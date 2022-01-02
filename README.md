
# fcl

<!-- badges: start -->
[![R-CMD-check](https://github.com/shrektan/fcl/workflows/R-CMD-check/badge.svg)](https://github.com/shrektan/fcl/actions)
<!-- badges: end -->

A financial calculator written in Rust. It provides simple calculations for bond YTM, Duration, etc.

It's also a illustration of how to use Rust code in an R package.

## Installation

You can install the development version of fcl from [GitHub](https://github.com/) with:

``` r
# install.packages("devtools")
devtools::install_github("shrektan/fcl")
```

## Example

This is a basic example which shows you how to solve a common problem:

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
```
