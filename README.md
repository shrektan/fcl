
<!-- README.md is generated from README.Rmd. Please edit that file -->
# fcl

<!-- badges: start -->
[![R-CMD-check](https://github.com/shrektan/fcl/workflows/R-CMD-check/badge.svg)](https://github.com/shrektan/fcl/actions) [![CRAN status](https://www.r-pkg.org/badges/version/fcl)](https://CRAN.R-project.org/package=fcl) [![Coverage Status](https://coveralls.io/repos/github/shrektan/fcl/badge.svg?branch=main)](https://coveralls.io/github/shrektan/fcl?branch=main) <!-- badges: end -->

A financial calculator written in Rust. It provides simple calculations for bond YTM, Duration, etc.

## Installation

You'll need the rust toolchain to compile this package from source.

## Example

``` r
library(fcl)
## basic example code
ymd <- ymd::ymd
bond_result(
  ymd(c(210101, 210101)),
  ymd(c(250101, 300201)),
  c(100.0, 100.0),
  c(0.05, 0.03),
  c(0L, 1L),
  ymd(c(220101, 220201)),
  c(100, 100)
)
#>          YTM     MACD     MODD
#> 1 0.04548481 3.000366 2.872103
#> 2 0.02997840 7.213824 7.007224
bond_cf(
  ymd(c(210101, 210201)),
  ymd(c(250101, 300201)),
  c(100.0, 100.0),
  c(0.05, 0.03),
  c(0L, 1L),
  ymd(c(220101, 220131))
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
rtn <- create_rtn(c(1, 1, 1), ymd(c(210101, 210105, 210110)), c(100, 123, 140), c(0, 3, 7))
rtn$twrr_cr(1, ymd(210102), ymd(210110))
#>                  [,1]
#> 2021-01-02 0.00000000
#> 2021-01-03 0.00000000
#> 2021-01-04 0.00000000
#> 2021-01-05 0.02500000
#> 2021-01-06 0.02500000
#> 2021-01-07 0.02500000
#> 2021-01-08 0.02500000
#> 2021-01-09 0.02500000
#> 2021-01-10 0.07894737
rtn$twrr_dr(1, ymd(210102), ymd(210110))
#>                  [,1]
#> 2021-01-02 0.00000000
#> 2021-01-03 0.00000000
#> 2021-01-04 0.00000000
#> 2021-01-05 0.02500000
#> 2021-01-06 0.00000000
#> 2021-01-07 0.00000000
#> 2021-01-08 0.00000000
#> 2021-01-09 0.00000000
#> 2021-01-10 0.05263158
rtn$dietz(1, ymd(210102), ymd(210110))
#>                  [,1]
#> 2021-01-02 0.00000000
#> 2021-01-03 0.00000000
#> 2021-01-04 0.00000000
#> 2021-01-05 0.02857143
#> 2021-01-06 0.02777778
#> 2021-01-07 0.02727273
#> 2021-01-08 0.02692308
#> 2021-01-09 0.02666667
#> 2021-01-10 0.08737864
rtn$dietz_avc(1, ymd(210102), ymd(210110))
#>                [,1]
#> 2021-01-02 100.0000
#> 2021-01-03 100.0000
#> 2021-01-04 100.0000
#> 2021-01-05 105.0000
#> 2021-01-06 108.0000
#> 2021-01-07 110.0000
#> 2021-01-08 111.4286
#> 2021-01-09 112.5000
#> 2021-01-10 114.4444
```
