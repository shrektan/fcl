#' @export
`[[.RFixedBond` <- `$.RFixedBond`

#' Create Fixed Bond Object
#' @param value_date,mty_date the value and maturity date of the bond
#' @param redem_value,cpn_rate,cpn_freq the redemption value, coupon rate and coupon frequency of the bond.
#'   Note that the **frequency** can only be one of 1, 2, 4, 0 (pay at mature)
#' @param ref_date,clean_price the reference date and the clean price that used to calculate the bond results
#' @details a double vector with 3 elements: ytm, macd and modd
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
