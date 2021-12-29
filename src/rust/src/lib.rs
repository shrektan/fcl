use extendr_api::prelude::*;
use chrono::{NaiveDate};
mod bond;

/// Calculate the Bond's YTM, Maclay Duration, Modified Duration
/// @param value_date,mty_date the value and maturity date of the bond
/// @param redem_value,cpn_rate,cpn_freq the redemption value, coupon rate and coupon frequency of the bond.
///   Note that the **frequency** can only be one of 1, 2, 4, 0 (pay at mature)
/// @return a double vector with 3 elements: ytm, macd and modd
/// @export
#[extendr]
fn bond_result(value_date: &str, mty_date: &str, redem_value: f64, cpn_rate: f64, cpn_freq: u32, ref_date: &str, clean_price: f64) -> Robj {
    let bond = bond::FixedBond::new(
        NaiveDate::parse_from_str(value_date, "%Y-%m-%d").unwrap(),
        NaiveDate::parse_from_str(mty_date, "%Y-%m-%d").unwrap(),
        redem_value,
        cpn_rate,
        cpn_freq
    );
    let ref_date = NaiveDate::parse_from_str(ref_date, "%Y-%m-%d").unwrap();
    let out = bond.result(&ref_date, clean_price);
    r!([out.ytm, out.macd, out.modd])
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod fcl;
    fn bond_result;
}
