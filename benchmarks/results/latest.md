# PySentry vs pip-audit Benchmark Report

**Generated:** 2026-10-16 14:46:27  
**Duration:** 41m 47.07s  
**Tools:** pip-audit 2.10.1, pysentry 0.5.0  
**Runs per config:** 5 (reported value is the **median**; ± is stdev)

## Methodology

Each configuration is run multiple times; the table shows the median execution time with min and stdev, median peak memory (summed over the process tree — both tools shell out to a resolver), and the vulnerability count each tool reported.

**Read speed alongside the vuln count.** The tools query different advisory databases, so a faster tool that reports fewer vulnerabilities is not strictly faster at the same work.

The vuln columns are **Distinct** (unique flaws after collapsing GHSA/PYSEC/CVE aliases — the real coverage, comparable across tools) and **Reported** (what the tool actually printed). A ⚠️ marks a Reported count higher than Distinct: the tool lists the same flaw under several advisory IDs. pysentry collapses them; pip-audit's OSV service does not, so it over-reports.

Every dataset is measured **cold** and **hot**. Cold clears all caches first — it is the fresh-CI-run experience (includes the vulnerability DB download and is network-sensitive). Hot has a warm DB and reflects steady-state throughput.

Most pysentry configs run with PEP 792 maintenance checks **off** (pip-audit has no equivalent, so this keeps the core comparison vuln-vs-vuln). The `pysentry-pypi-maintenance` config leaves them **on** so the feature's cost is visible.

### Datasets

| Dataset | Packages | Mode |
|---------|----------|------|
| small_requirements | 10 | resolve |
| large_requirements | 55 | resolve |
| pinned_requirements | 185 | audit-only (pinned, reproducible) |
| pinned_large_requirements | 692 | audit-only (pinned, reproducible) |

- **resolve** — unpinned specs, dependency resolution ON. Includes resolver cost; pysentry uses `uv`, pip-audit uses `pip`. Not pinned, so results drift as the ecosystem and advisory DB change over time.
- **audit-only** — pinned input, resolution OFF on both tools (`--no-resolver` / `--no-deps --disable-pip`). This isolates audit throughput from resolver choice and is reproducible.

### Commands

| Config | Command |
|--------|---------|
| pysentry-pypa | `pysentry --format json --resolver uv --sources pypa --no-maintenance-check <path>` |
| pysentry-osv | `pysentry --format json --resolver uv --sources osv --no-maintenance-check <path>` |
| pysentry-pypi | `pysentry --format json --resolver uv --sources pypi --no-maintenance-check <path>` |
| pysentry-all-sources | `pysentry --format json --resolver uv --sources pypa,osv,pypi --no-maintenance-check <path>` |
| pysentry-pypi-maintenance | `pysentry --format json --resolver uv --sources pypi <path>` |
| pip-audit-pypi | `pip-audit --format json -s pypi --requirement <path>` |
| pip-audit-osv | `pip-audit --format json -s osv --requirement <path>` |

## Test Environment

- **Platform:** Linux-6.17.0-1022-azure-x86_64-with-glibc2.39
- **Python:** 3.11.17
- **CPU Cores:** 4
- **Total Memory:** 15.61 GB

## Performance

### small_requirements — cold cache

| Config | Median Time | Min | Stdev | Rel | Peak Mem | Distinct | Reported |
|--------|-------------|-----|-------|-----|----------|----------|----------|
| 🥇 pysentry-pypi | 0.264s | 0.255s | ±0.031s | 1.00x | 120.39 MB | 0 | 0 |
| 🥈 pysentry-pypi-maintenance | 0.319s | 0.313s | ±0.022s | 1.20x | 119.39 MB | 0 | 0 |
| pysentry-osv | 0.406s | 0.401s | ±0.052s | 1.53x | 117.78 MB | 0 | 0 |
| pysentry-pypa | 2.076s | 2.031s | ±0.469s | 7.85x | 286.37 MB | 0 | 0 |
| pysentry-all-sources | 2.096s | 2.052s | ±0.036s | 7.92x | 288.03 MB | 0 | 0 |
| pip-audit-pypi | 8.312s | 8.244s | ±0.123s | 31.43x | 138.50 MB | 0 | 0 |
| pip-audit-osv | 10.696s | 10.526s | ±0.230s | 40.44x | 138.47 MB | 0 | 0 |

### small_requirements — hot cache

| Config | Median Time | Min | Stdev | Rel | Peak Mem | Distinct | Reported |
|--------|-------------|-----|-------|-----|----------|----------|----------|
| 🥇 pysentry-osv | 0.194s | 0.192s | ±0.002s | 1.00x | 120.40 MB | 0 | 0 |
| 🥈 pysentry-pypi | 0.195s | 0.185s | ±0.015s | 1.01x | 120.04 MB | 0 | 0 |
| pysentry-pypi-maintenance | 0.212s | 0.197s | ±0.007s | 1.09x | 118.57 MB | 0 | 0 |
| pysentry-pypa | 0.959s | 0.948s | ±0.007s | 4.94x | 386.57 MB | 0 | 0 |
| pysentry-all-sources | 0.988s | 0.986s | ±0.014s | 5.08x | 386.82 MB | 0 | 0 |
| pip-audit-pypi | 7.467s | 7.447s | ±0.076s | 38.42x | 135.36 MB | 0 | 0 |
| pip-audit-osv | 10.161s | 10.029s | ±0.314s | 52.29x | 137.66 MB | 0 | 0 |

### large_requirements — cold cache

| Config | Median Time | Min | Stdev | Rel | Peak Mem | Distinct | Reported |
|--------|-------------|-----|-------|-----|----------|----------|----------|
| 🥇 pysentry-pypi | 0.694s | 0.677s | ±0.013s | 1.00x | 216.07 MB | 0 | 0 |
| 🥈 pysentry-pypi-maintenance | 0.841s | 0.823s | ±0.030s | 1.21x | 214.35 MB | 0 | 0 |
| pysentry-osv | 0.907s | 0.878s | ±0.017s | 1.31x | 214.52 MB | 0 | 0 |
| pysentry-all-sources | 2.497s | 2.474s | ±0.042s | 3.60x | 289.11 MB | 0 | 0 |
| pysentry-pypa | 2.505s | 2.474s | ±0.035s | 3.61x | 286.38 MB | 0 | 0 |
| pip-audit-pypi | 16.320s | 15.386s | ±0.785s | 23.51x | 212.19 MB | 0 | 0 |
| pip-audit-osv | 30.146s | 29.916s | ±2.084s | 43.43x | 212.29 MB | 0 | 0 |

### large_requirements — hot cache

| Config | Median Time | Min | Stdev | Rel | Peak Mem | Distinct | Reported |
|--------|-------------|-----|-------|-----|----------|----------|----------|
| 🥇 pysentry-pypi | 0.559s | 0.548s | ±0.017s | 1.00x | 215.62 MB | 0 | 0 |
| 🥈 pysentry-osv | 0.562s | 0.557s | ±0.015s | 1.01x | 215.12 MB | 0 | 0 |
| pysentry-pypi-maintenance | 0.584s | 0.573s | ±0.009s | 1.04x | 212.34 MB | 0 | 0 |
| pysentry-pypa | 1.329s | 1.315s | ±0.019s | 2.38x | 386.62 MB | 0 | 0 |
| pysentry-all-sources | 1.331s | 1.309s | ±0.013s | 2.38x | 386.85 MB | 0 | 0 |
| pip-audit-pypi | 13.329s | 13.298s | ±0.118s | 23.84x | 211.02 MB | 0 | 0 |
| pip-audit-osv | 28.576s | 28.104s | ±0.449s | 51.11x | 210.77 MB | 0 | 0 |

### pinned_requirements — cold cache

| Config | Median Time | Min | Stdev | Rel | Peak Mem | Distinct | Reported |
|--------|-------------|-----|-------|-----|----------|----------|----------|
| 🥇 pysentry-pypi | 0.217s | 0.205s | ±0.102s | 1.00x | 14.51 MB | 120 | 120 |
| 🥈 pysentry-pypi-maintenance | 0.320s | 0.312s | ±0.264s | 1.48x | 45.86 MB | 120 | 120 |
| pysentry-pypa | 1.947s | 1.899s | ±0.208s | 8.99x | 286.30 MB | 113 | 113 |
| pysentry-osv | 2.288s | 2.162s | ±0.153s | 10.57x | 22.77 MB | 120 | 120 |
| pip-audit-pypi | 2.597s | 2.003s | ±0.531s | 12.00x | 51.23 MB | 120 | 219 ⚠️ |
| pysentry-all-sources | 3.677s | 3.635s | ±1.093s | 16.98x | 297.64 MB | 121 | 121 |
| pip-audit-osv | 25.062s | 24.072s | ±1.024s | 115.74x | 51.42 MB | 120 | 230 ⚠️ |

### pinned_requirements — hot cache

| Config | Median Time | Min | Stdev | Rel | Peak Mem | Distinct | Reported |
|--------|-------------|-----|-------|-----|----------|----------|----------|
| 🥇 pysentry-pypi | 0.020s | 0.020s | ±0.001s | 1.00x | 7.60 MB | 120 | 120 |
| 🥈 pysentry-pypi-maintenance | 0.034s | 0.031s | ±0.005s | 1.69x | 7.65 MB | 120 | 120 |
| pysentry-osv | 0.053s | 0.052s | ±0.001s | 2.61x | 7.62 MB | 120 | 120 |
| pip-audit-pypi | 0.757s | 0.752s | ±0.006s | 37.13x | 51.21 MB | 120 | 219 ⚠️ |
| pysentry-pypa | 0.800s | 0.799s | ±0.010s | 39.24x | 386.48 MB | 113 | 113 |
| pysentry-all-sources | 0.836s | 0.828s | ±0.012s | 40.99x | 400.32 MB | 121 | 121 |
| pip-audit-osv | 27.063s | 26.472s | ±0.304s | 1326.78x | 56.82 MB | 120 | 230 ⚠️ |

### pinned_large_requirements — cold cache

| Config | Median Time | Min | Stdev | Rel | Peak Mem | Distinct | Reported |
|--------|-------------|-----|-------|-----|----------|----------|----------|
| 🥇 pysentry-pypi | 0.486s | 0.418s | ±0.356s | 1.00x | 15.97 MB | 95 | 95 |
| 🥈 pysentry-pypi-maintenance | 0.725s | 0.687s | ±0.344s | 1.49x | 55.62 MB | 95 | 95 |
| pysentry-pypa | 1.903s | 1.889s | ±0.116s | 3.92x | 286.91 MB | 91 | 91 |
| pysentry-osv | 2.614s | 2.402s | ±0.146s | 5.38x | 27.27 MB | 98 | 98 |
| pysentry-all-sources | 3.415s | 3.372s | ±0.395s | 7.03x | 300.14 MB | 98 | 98 |
| pip-audit-pypi | 8.784s | 5.397s | ±2.033s | 18.08x | 52.52 MB | 95 | 126 ⚠️ |
| pip-audit-osv | 1m 38.09s | 1m 37.62s | ±3.170s | 201.96x | 52.73 MB | 98 | 184 ⚠️ |

### pinned_large_requirements — hot cache

| Config | Median Time | Min | Stdev | Rel | Peak Mem | Distinct | Reported |
|--------|-------------|-----|-------|-----|----------|----------|----------|
| 🥇 pysentry-pypi | 0.027s | 0.026s | ±0.001s | 1.00x | 7.61 MB | 95 | 95 |
| 🥈 pysentry-pypi-maintenance | 0.048s | 0.043s | ±0.006s | 1.76x | 7.62 MB | 95 | 95 |
| pysentry-osv | 0.091s | 0.090s | ±0.001s | 3.34x | 26.39 MB | 98 | 98 |
| pysentry-pypa | 0.813s | 0.802s | ±0.008s | 29.89x | 386.48 MB | 91 | 91 |
| pysentry-all-sources | 0.904s | 0.900s | ±0.004s | 33.26x | 405.46 MB | 98 | 98 |
| pip-audit-pypi | 1.457s | 1.443s | ±0.012s | 53.58x | 51.40 MB | 95 | 126 ⚠️ |
| pip-audit-osv | 1m 40.64s | 1m 38.46s | ±23.366s | 3700.28x | 52.64 MB | 98 | 184 ⚠️ |

## Choosing a pysentry configuration

Concrete cost of each pysentry option. `vs best` is relative to the fastest pysentry config on that row. Source choice (`pypa` / `osv` / `pypi` / `all-sources`) trades coverage against speed and memory; `pysentry-pypi-maintenance` is the only config with PEP 792 maintenance checks on, so its delta is that feature's cost.

### small_requirements — cold cache

| Config | Median Time | Peak Mem | Distinct | Reported | vs best |
|--------|-------------|----------|----------|----------|---------|
| pysentry-pypi | 0.264s | 120.39 MB | 0 | 0 | 1.00x |
| pysentry-pypi-maintenance | 0.319s | 119.39 MB | 0 | 0 | 1.20x |
| pysentry-osv | 0.406s | 117.78 MB | 0 | 0 | 1.53x |
| pysentry-pypa | 2.076s | 286.37 MB | 0 | 0 | 7.85x |
| pysentry-all-sources | 2.096s | 288.03 MB | 0 | 0 | 7.92x |

### small_requirements — hot cache

| Config | Median Time | Peak Mem | Distinct | Reported | vs best |
|--------|-------------|----------|----------|----------|---------|
| pysentry-osv | 0.194s | 120.40 MB | 0 | 0 | 1.00x |
| pysentry-pypi | 0.195s | 120.04 MB | 0 | 0 | 1.01x |
| pysentry-pypi-maintenance | 0.212s | 118.57 MB | 0 | 0 | 1.09x |
| pysentry-pypa | 0.959s | 386.57 MB | 0 | 0 | 4.94x |
| pysentry-all-sources | 0.988s | 386.82 MB | 0 | 0 | 5.08x |

### large_requirements — cold cache

| Config | Median Time | Peak Mem | Distinct | Reported | vs best |
|--------|-------------|----------|----------|----------|---------|
| pysentry-pypi | 0.694s | 216.07 MB | 0 | 0 | 1.00x |
| pysentry-pypi-maintenance | 0.841s | 214.35 MB | 0 | 0 | 1.21x |
| pysentry-osv | 0.907s | 214.52 MB | 0 | 0 | 1.31x |
| pysentry-all-sources | 2.497s | 289.11 MB | 0 | 0 | 3.60x |
| pysentry-pypa | 2.505s | 286.38 MB | 0 | 0 | 3.61x |

### large_requirements — hot cache

| Config | Median Time | Peak Mem | Distinct | Reported | vs best |
|--------|-------------|----------|----------|----------|---------|
| pysentry-pypi | 0.559s | 215.62 MB | 0 | 0 | 1.00x |
| pysentry-osv | 0.562s | 215.12 MB | 0 | 0 | 1.01x |
| pysentry-pypi-maintenance | 0.584s | 212.34 MB | 0 | 0 | 1.04x |
| pysentry-pypa | 1.329s | 386.62 MB | 0 | 0 | 2.38x |
| pysentry-all-sources | 1.331s | 386.85 MB | 0 | 0 | 2.38x |

### pinned_requirements — cold cache

| Config | Median Time | Peak Mem | Distinct | Reported | vs best |
|--------|-------------|----------|----------|----------|---------|
| pysentry-pypi | 0.217s | 14.51 MB | 120 | 120 | 1.00x |
| pysentry-pypi-maintenance | 0.320s | 45.86 MB | 120 | 120 | 1.48x |
| pysentry-pypa | 1.947s | 286.30 MB | 113 | 113 | 8.99x |
| pysentry-osv | 2.288s | 22.77 MB | 120 | 120 | 10.57x |
| pysentry-all-sources | 3.677s | 297.64 MB | 121 | 121 | 16.98x |

### pinned_requirements — hot cache

| Config | Median Time | Peak Mem | Distinct | Reported | vs best |
|--------|-------------|----------|----------|----------|---------|
| pysentry-pypi | 0.020s | 7.60 MB | 120 | 120 | 1.00x |
| pysentry-pypi-maintenance | 0.034s | 7.65 MB | 120 | 120 | 1.69x |
| pysentry-osv | 0.053s | 7.62 MB | 120 | 120 | 2.61x |
| pysentry-pypa | 0.800s | 386.48 MB | 113 | 113 | 39.24x |
| pysentry-all-sources | 0.836s | 400.32 MB | 121 | 121 | 40.99x |

### pinned_large_requirements — cold cache

| Config | Median Time | Peak Mem | Distinct | Reported | vs best |
|--------|-------------|----------|----------|----------|---------|
| pysentry-pypi | 0.486s | 15.97 MB | 95 | 95 | 1.00x |
| pysentry-pypi-maintenance | 0.725s | 55.62 MB | 95 | 95 | 1.49x |
| pysentry-pypa | 1.903s | 286.91 MB | 91 | 91 | 3.92x |
| pysentry-osv | 2.614s | 27.27 MB | 98 | 98 | 5.38x |
| pysentry-all-sources | 3.415s | 300.14 MB | 98 | 98 | 7.03x |

### pinned_large_requirements — hot cache

| Config | Median Time | Peak Mem | Distinct | Reported | vs best |
|--------|-------------|----------|----------|----------|---------|
| pysentry-pypi | 0.027s | 7.61 MB | 95 | 95 | 1.00x |
| pysentry-pypi-maintenance | 0.048s | 7.62 MB | 95 | 95 | 1.76x |
| pysentry-osv | 0.091s | 26.39 MB | 98 | 98 | 3.34x |
| pysentry-pypa | 0.813s | 386.48 MB | 91 | 91 | 29.89x |
| pysentry-all-sources | 0.904s | 405.46 MB | 98 | 98 | 33.26x |