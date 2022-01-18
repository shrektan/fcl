#' @export
`[[.RFixedBond` <- `$.RFixedBond`

#' Create Fixed Bond Object
#' @param value_date,mty_date the value and maturity date of the bond
#' @param redem_value,cpn_rate,cpn_freq the redemption value, coupon rate and coupon frequency of the bond.
#'   Note that the **frequency** can only be one of 1, 2, 4, 0 (pay at mature)
#' @note
#'   * all arguments must be the same length or 1.
#'   * The date input will be converted to Date object via [ymd::ymd()].
#' @details it returns an environment containing the following objects:
#'   * `.self`: an external pointer of the Rust object.
#'   * `len()`: a function returns the length of the internal bonds object.
#'   * `ytm_dur(ref_date, clean_price)`: a function returns a data.frame, with three columns,
#'      YTM, MODD and MACD.
#'   * `cf(ref_date)`: a function returns the schedualed bond cashflows, in `xts` format.
#' @note
#'   * It doesn't take the day count convention into account for now.
#'   * There's no support for business day calendar. The dates in the cashflow projection are the
#'     same days in the next few months (see [ymd::edate()]). It considers different days in each
#'     month but no weekend date adjustment.
#'   * The YTM value is the cashflow's IRR value. Thus, it doesn't equal to the Excel's Yield
#'     value, which is adjusted using this formula
#'     \eqn{YTM (fcl) = (1 + frac{Yield (Excel)}{n})^n - 1},
#'     where n is the the coupon payment frequency, when the remaining life of the bond is larger
#'     than 1.
#'   * When the bond is going to mature within one year, the \eqn{Yield (Excel) = frac{Cashflow}{Price} - 1}.
#'
#' @export
fixed_bond <- function(value_date, mty_date, redem_value, cpn_rate, cpn_freq) {
  args <- prepare_args(
    ymd(value_date), ymd(mty_date), as.double(redem_value), as.double(cpn_rate), as.integer(cpn_freq)
  )
  out <- new.env()
  out$.self <- do.call(RFixedBond$new, args)
  out$len <- function() {
    out$.self$len()
  }
  out$ytm_dur <- function(ref_date, clean_price) {
    args <- prepare_args(
      ref_date = ymd(ref_date), clean_price = as.double(clean_price), .len = out$len()
    )
    with(args, out$.self$ytm_dur(ref_date, clean_price))
  }
  out$cf <- function(ref_date) {
    args <- prepare_args(
      ref_date = ymd(ref_date), .len = out$len()
    )
    with(args, out$.self$cf(ref_date))
  }
  out
}
