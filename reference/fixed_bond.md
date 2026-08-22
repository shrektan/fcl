# Create Fixed Bond Object

Create Fixed Bond Object

## Usage

``` r
fixed_bond(value_date, mty_date, redem_value, cpn_rate, cpn_freq)
```

## Arguments

- value_date, mty_date:

  the value and maturity date of the bond

- redem_value, cpn_rate, cpn_freq:

  the redemption value, coupon rate and coupon frequency of the bond.
  Note that the **frequency** can only be one of 1, 2, 4, 0 (pay at
  mature)

## Value

it returns an environment containing the following objects:

- `.self`: an external pointer of the Rust object.

- `len()`: a function returns the length of the internal bonds object.

- `ytm_dur(ref_date, clean_price)`: a function returns a data.frame,
  with three columns, 'YTM' (Yield to Maturity), 'MODD' (Modified
  Duration) and 'MACD' (Macaulay Duration).

- `cf(ref_date)`: a function that returns the scheduled bond cash flows,
  in `xts` format.

## Note

- all arguments must be the same length or 1.

- The date input will be converted to Date object via
  [`ymd::ymd()`](https://shrektan.github.io/ymd/reference/ymd.html).

&nbsp;

- It doesn't take the day count convention into account for now.

- There's no support for business day calendar. The dates in the cash
  flow projection are the same days in the next few months (see
  [`ymd::edate()`](https://shrektan.github.io/ymd/reference/edate.html)).
  It considers different days in each month but no weekend date
  adjustment.

- The 'YTM' value is the cash flow's 'IRR' (internal rate of return)
  value. Thus, it doesn't equal to the Yield value returned by Excel,
  which is adjusted using this formula \\YTM (fcl) = (1 + frac{Yield
  (Excel)}{n})^n - 1\\, where n is the coupon payment frequency, when
  the remaining life of the bond is larger than 1.

- When the bond is going to mature within one year, the \\Yield (Excel)
  = frac{Cash flow}{Price} - 1\\.

## Examples

``` r
bond <- fixed_bond(
  value_date = 210101,
  mty_date = c(250101, 300201),
  redem_value = 100,
  cpn_rate = c(0.05, 0.03),
  cpn_freq = c(0, 1)
)
bond$ytm_dur(
  ref_date = c(220101, 220201),
  clean_price = 100
)
#>          YTM     MACD     MODD
#> 1 0.04552728 3.000000 2.869366
#> 2 0.02999755 7.212793 7.002728
bond$cf(
  ref_date = c(220101, 220131)
)
#>    ID       DATE     COUPON REDEM
#> 1   1 2025-01-01 20.0000000   100
#> 2   2 2023-01-01  3.0000000     0
#> 3   2 2024-01-01  3.0000000     0
#> 4   2 2025-01-01  3.0000000     0
#> 5   2 2026-01-01  3.0000000     0
#> 6   2 2027-01-01  3.0000000     0
#> 7   2 2028-01-01  3.0000000     0
#> 8   2 2029-01-01  3.0000000     0
#> 9   2 2030-01-01  3.0000000     0
#> 10  2 2030-02-01  0.2547945   100
```
