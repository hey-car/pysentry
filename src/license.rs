// SPDX-License-Identifier: MIT

use crate::config::HttpConfig;
use crate::dependency::scanner::{DependencySource, ScannedDependency};
use crate::{AuditError, Result};
use futures::future::join_all;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

const PYPI_JSON_BASE_URL: &str = "https://pypi.org/pypi";
const MAX_CONCURRENT_REQUESTS: usize = 15;

#[derive(Debug, Clone, serde::Serialize)]
pub struct LicenseIssue {
    pub package_name: crate::types::PackageName,
    pub installed_version: crate::types::Version,
    pub license: String,
    pub is_direct: bool,
    pub source_file: Option<String>,
}

#[derive(Deserialize)]
struct PackageResponse {
    info: PackageInfo,
}

#[derive(Deserialize)]
struct PackageInfo {
    #[serde(default)]
    license_expression: Option<String>,
    #[serde(default)]
    classifiers: Vec<String>,
}

pub struct LicenseClient {
    client: Client,
    semaphore: Arc<Semaphore>,
}

impl LicenseClient {
    pub fn new(http_config: &HttpConfig) -> Result<Self> {
        let client = Client::builder()
            .user_agent(format!("pysentry/{}", env!("CARGO_PKG_VERSION")))
            .timeout(Duration::from_secs(http_config.timeout))
            .connect_timeout(Duration::from_secs(http_config.connect_timeout))
            .build()
            .map_err(AuditError::Http)?;
        Ok(Self {
            client,
            semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS)),
        })
    }

    pub async fn find_copyleft_licenses(
        &self,
        dependencies: &[ScannedDependency],
    ) -> Result<Vec<LicenseIssue>> {
        let checks = dependencies
            .iter()
            .filter(|dependency| matches!(dependency.source, DependencySource::Registry))
            .map(|dependency| self.check_package(dependency));
        let mut issues = Vec::new();
        for result in join_all(checks).await {
            match result {
                Ok(Some(issue)) => issues.push(issue),
                Ok(None) => {}
                Err(error) => return Err(error),
            }
        }
        Ok(issues)
    }

    async fn check_package(&self, dependency: &ScannedDependency) -> Result<Option<LicenseIssue>> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|error| AuditError::other(format!("Semaphore acquire failed: {error}")))?;
        let url = format!(
            "{}/{}/{}/json",
            PYPI_JSON_BASE_URL, dependency.name, dependency.version
        );
        let response = self.client.get(&url).send().await?.error_for_status()?;
        let package: PackageResponse = response.json().await?;

        let license = package
            .info
            .license_expression
            .as_deref()
            .and_then(copyleft_spdx_identifier)
            .map(str::to_string)
            .or_else(|| {
                package.info.classifiers.iter().find_map(|classifier| {
                    copyleft_classifier_identifier(classifier).map(str::to_string)
                })
            });

        Ok(license.map(|license| LicenseIssue {
            package_name: dependency.name.clone(),
            installed_version: dependency.version.clone(),
            license,
            is_direct: dependency.is_direct,
            source_file: dependency.source_file.clone(),
        }))
    }
}

fn copyleft_spdx_identifier(expression: &str) -> Option<&str> {
    expression
        .split(|character: char| {
            character.is_ascii_whitespace() || matches!(character, '(' | ')' | '+')
        })
        .find(|identifier| {
            let uppercase = identifier.to_ascii_uppercase();
            uppercase == "GPL"
                || uppercase.starts_with("GPL-")
                || uppercase == "AGPL"
                || uppercase.starts_with("AGPL-")
        })
}

fn copyleft_classifier_identifier(classifier: &str) -> Option<&'static str> {
    let lowercase = classifier.to_ascii_lowercase();
    if lowercase.contains("gnu affero general public license") {
        Some("AGPL")
    } else if lowercase.contains("gnu general public license") {
        Some("GPL")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{copyleft_classifier_identifier, copyleft_spdx_identifier};

    #[test]
    fn detects_strong_copyleft_spdx_identifiers() {
        assert_eq!(
            copyleft_spdx_identifier("GPL-3.0-or-later"),
            Some("GPL-3.0-or-later")
        );
        assert_eq!(
            copyleft_spdx_identifier("MIT OR AGPL-3.0-only"),
            Some("AGPL-3.0-only")
        );
    }

    #[test]
    fn excludes_weak_and_permissive_spdx_identifiers() {
        assert_eq!(copyleft_spdx_identifier("LGPL-3.0-only"), None);
        assert_eq!(copyleft_spdx_identifier("MIT"), None);
    }

    #[test]
    fn detects_strong_copyleft_classifiers() {
        assert_eq!(
            copyleft_classifier_identifier(
                "License :: OSI Approved :: GNU General Public License v3 (GPLv3)"
            ),
            Some("GPL")
        );
        assert_eq!(
            copyleft_classifier_identifier(
                "License :: OSI Approved :: GNU Affero General Public License v3"
            ),
            Some("AGPL")
        );
    }
}
