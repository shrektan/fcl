use extendr_api::prelude::*;
use chrono::{NaiveDate};
mod bond;

fn robj2date(x: Robj) -> NaiveDate {
    if !x.inherits("Date") || x.len() != 1 {
        panic!("must be a scalar Date");
    } else {
        let days = x.as_real().expect("must not be NA") as i32;
        NaiveDate::from_num_days_from_ce(days + 719163)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn to_date() {
        test! {
            single_threaded(|| {
                let x: Robj = r!(18990.0).set_class(&["Date"]).unwrap();
                assert_eq!(robj2date(x), NaiveDate::from_ymd(2021, 12, 29));
            });
        }
    }
}

/// Calculate the Bond's YTM, Maclay Duration, Modified Duration
/// @param value_date,mty_date the value and maturity date of the bond
/// @param redem_value,cpn_rate,cpn_freq the redemption value, coupon rate and coupon frequency of the bond.
///   Note that the **frequency** can only be one of 1, 2, 4, 0 (pay at mature)
/// @return a double vector with 3 elements: ytm, macd and modd
/// @export
#[extendr]
fn bond_result(value_date: Robj, mty_date: Robj, redem_value: f64, cpn_rate: f64, cpn_freq: u32, ref_date: Robj, clean_price: f64) -> Robj {
    let bond = bond::FixedBond::new(
        robj2date(value_date),
        // NaiveDate::parse_from_str(value_date, "%Y-%m-%d").unwrap(),
        robj2date(mty_date),
        redem_value,
        cpn_rate,
        cpn_freq
    );
    let ref_date = robj2date(ref_date);
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
