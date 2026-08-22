# R CMD check results

0 errors | 0 warnings | 0 notes

## Test environments

* Local: R 4.6.1, macOS 26.5.2, arm64
* GitHub Actions: R release and devel on Windows
* GitHub Actions: R release on macOS
* GitHub Actions: R release and devel on Ubuntu
* GitHub Actions: R release with Rust nightly on Ubuntu

## Resubmission

* Fixes the `R_NamespaceRegistry` non-API entry-point issue reported by CRAN by
  upgrading the `extendr` Rust crates.
* Rust dependencies are vendored and package installation builds them offline.

## Reverse dependencies

There are currently no reverse dependencies on CRAN.
