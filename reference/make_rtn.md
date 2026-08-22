# Create a Return Object

By providing a "group" (`ids`) of `dates`, `mvs` and `pls`, calculating
the Time-weighted Rate of Return (TWRR) or Modified Dietz Rate of Return
(DIETZ).

## Usage

``` r
make_rtn(date, mv, pl, id = 1L)
```

## Arguments

- date:

  a Date vector, the reference date of each row

- mv, pl:

  a double vector, the market value and the 'PnL' (Profit and Loss) of
  each day

- id:

  an integer vector, the ID of each row belongs to

## Value

A list of functions, with signature of `from`, `to` and `id`, all of
which are only allowed to accept a scalar. They all return an `xts`
object with one column.

- `twrr_cr`: the cumulative Time-weighted Return

- `twrr_dr`: the daily Time-weighted Return

- `dietz`: the Modified Dietz Return

- `dietz_avc`: the denominator used to calculate the Modified Dietz
  Return

- `cum_pl`: the cumulative PnL

## Note

All the input vector must be 1 or the same length.

## Cash flow handling

- The cash flow is not provided externally. Instead, it's deducted via
  market value and PnL, with the equation \\\Delta MV = \Delta PnL +
  CF\\.

- The cash inflow is treating as if it happens at the beginning of the
  day, while the cash outflow is at the end of the day. The reasons are
  two. The first is to reduce the possibility of having a close-to-zero
  denominator. The second is the cash is usually not usable for the
  whole day.

- The calculation is based on calendar days. No business calendar or
  weekday considers. You can't change the calculation frequency, either.
  However, this is possible in the future version.

## References

Modified Dietz Method:
https://en.wikipedia.org/wiki/Modified_Dietz_method

Time weighed Return: https://en.wikipedia.org/wiki/Time-weighted_return

## Examples

``` r
rtn <- make_rtn(date = c(210101, 210105, 210110), mv = c(100, 123, 140), pl = c(0, 3, 7))
rtn$twrr_cr(210102, 210110)
#>               TWRR_CR
#> 2021-01-02 0.00000000
#> 2021-01-03 0.00000000
#> 2021-01-04 0.00000000
#> 2021-01-05 0.02500000
#> 2021-01-06 0.02500000
#> 2021-01-07 0.02500000
#> 2021-01-08 0.02500000
#> 2021-01-09 0.02500000
#> 2021-01-10 0.07894737
rtn$twrr_dr(210102, 210110)
#>               TWRR_DR
#> 2021-01-02 0.00000000
#> 2021-01-03 0.00000000
#> 2021-01-04 0.00000000
#> 2021-01-05 0.02500000
#> 2021-01-06 0.00000000
#> 2021-01-07 0.00000000
#> 2021-01-08 0.00000000
#> 2021-01-09 0.00000000
#> 2021-01-10 0.05263158
rtn$dietz(210102, 210110)
#>                 DIETZ
#> 2021-01-02 0.00000000
#> 2021-01-03 0.00000000
#> 2021-01-04 0.00000000
#> 2021-01-05 0.02857143
#> 2021-01-06 0.02777778
#> 2021-01-07 0.02727273
#> 2021-01-08 0.02692308
#> 2021-01-09 0.02666667
#> 2021-01-10 0.08737864
rtn$dietz_avc(210102, 210110)
#>            DIETZ_AVC
#> 2021-01-02  100.0000
#> 2021-01-03  100.0000
#> 2021-01-04  100.0000
#> 2021-01-05  105.0000
#> 2021-01-06  108.0000
#> 2021-01-07  110.0000
#> 2021-01-08  111.4286
#> 2021-01-09  112.5000
#> 2021-01-10  114.4444
```
