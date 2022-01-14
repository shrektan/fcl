#' Calculate the Bond's YTM, Maclay Duration, Modified Duration
#' @param value_date,mty_date the value and maturity date of the bond
#' @param redem_value,cpn_rate,cpn_freq the redemption value, coupon rate and coupon frequency of the bond.
#'   Note that the **frequency** can only be one of 1, 2, 4, 0 (pay at mature)
#' @param ref_date,clean_price the reference date and the clean price that used to calculate the bond results
#' @return a double vector with 3 elements: ytm, macd and modd
#' @export
bond_result <- function(value_date, mty_date, redem_value, cpn_rate, cpn_freq, ref_date, clean_price) {
  rust_bond_result(value_date, mty_date, redem_value, cpn_rate, cpn_freq, ref_date, clean_price)
}


#' Generate bond's cash flows
#' @inheritParams bond_result
#' @export
bond_cf <- function(value_date, mty_date, redem_value, cpn_rate, cpn_freq, ref_date) {
  rust_bond_cf(value_date, mty_date, redem_value, cpn_rate, cpn_freq, ref_date)
}
