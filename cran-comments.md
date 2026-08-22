# R CMD check results

0 errors | 0 warnings | 0 notes

## Test environments

* Local, exact submitted tarball: R 4.6.1, macOS 26.5.2, arm64
* win-builder, exact submitted tarball: R-devel (2026-08-21 r90440,
  UCRT), Windows Server 2022 x64
* GitHub Actions: R release and devel on Windows
* GitHub Actions: R release on macOS
* GitHub Actions: R release and devel on Ubuntu
* GitHub Actions: R release with Rust nightly on Ubuntu
* Official macOS builder: the release flavor was unavailable. The only offered
  development flavor returned HTTP 502 on the upload and one bounded retry.
  Local current-release and GitHub Actions macOS release checks passed.

## Resubmission

* Fixes the `R_NamespaceRegistry` non-API entry-point issue reported by CRAN by
  upgrading the `extendr` Rust crates.
* Rust dependencies are vendored and package installation builds them offline.

## Reverse dependencies

There are currently no reverse dependencies on CRAN.
