
<!-- README.md is generated from README.Rmd. Please edit that file -->
# fcl

<!-- badges: start -->
[![R-CMD-check](https://github.com/shrektan/fcl/workflows/R-CMD-check/badge.svg)](https://github.com/shrektan/fcl/actions) [![CRAN status](https://www.r-pkg.org/badges/version/fcl)](https://CRAN.R-project.org/package=fcl) <!-- badges: end -->

A financial calculator written in Rust. It provides simple calculations for bond YTM, Duration, etc.

## Installation

You'll need the rust toolchain to compile this package from source.

## Example

``` r
library(fcl)
## basic example code
ymd <- function(...) as.Date(as.character(list(...)))
bond_result(
  ymd("2021-01-01", "2021-02-01"),
  ymd("2025-01-01", "2030-02-01"),
  c(100.0, 100.0),
  c(0.05, 0.03),
  c(0L, 1L),
  ymd("2022-01-01", "2022-02-01"),
  c(100, 100)
)
#>          ytm     macd     modd
#> 1 0.04548481 3.000366 2.872103
#> 2 0.02997948 7.231391 7.024557
bond_cf(
  ymd("2021-01-01", "2021-02-01"),
  ymd("2025-01-01", "2030-02-01"),
  c(100.0, 100.0),
  c(0.05, 0.03),
  c(0L, 1L)
)
#>    ID       DATE COUPON REDEM
#> 1   1 2025-01-01     20   100
#> 2   2 2022-02-01      3     0
#> 3   2 2023-02-01      3     0
#> 4   2 2024-02-01      3     0
#> 5   2 2025-02-01      3     0
#> 6   2 2026-02-01      3     0
#> 7   2 2027-02-01      3     0
#> 8   2 2028-02-01      3     0
#> 9   2 2029-02-01      3     0
#> 10  2 2030-02-01      3   100
```
