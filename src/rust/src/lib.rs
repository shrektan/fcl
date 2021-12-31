use chrono::NaiveDate;
use extendr_api::prelude::*;
mod bond;

fn robj2date(x: Robj, var: &str) -> Vec<Option<NaiveDate>> {
    if !x.inherits("Date") || x.rtype() != RType::Real {
        panic!("`{}` must be Date and use double value", &var)
    } else {
        let days = x.as_real_iter().unwrap();
        days.map(|d| {
            if d.is_na() {
                None
            } else {
                NaiveDate::from_num_days_from_ce_opt(d as i32 + 719163)
            }
        })
        .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn to_date() {
        test! {
            single_threaded(|| {
                let x: Robj = r!([18990.0, 18991.0]).set_class(&["Date"]).unwrap();
                assert_eq!(robj2date(x, "x"), [Some(NaiveDate::from_ymd(2021, 12, 29)), Some(NaiveDate::from_ymd(2021, 12, 30))]);
            });
        }
    }
}

/// Calculate the Bond's YTM, Maclay Duration, Modified Duration
/// @param value_date,mty_date the value and maturity date of the bond
/// @param redem_value,cpn_rate,cpn_freq the redemption value, coupon rate and coupon frequency of the bond.
///   Note that the **frequency** can only be one of 1, 2, 4, 0 (pay at mature)
/// @param ref_date,clean_price the reference date and the clean price that used to calculate the bond results
/// @return a double vector with 3 elements: ytm, macd and modd
/// @export
#[extendr]
fn bond_result(
    value_date: Robj,
    mty_date: Robj,
    redem_value: Robj,
    cpn_rate: Robj,
    cpn_freq: Robj,
    ref_date: Robj,
    clean_price: Robj,
) -> Robj {
    let n = value_date.len();
    let check = |v: &Robj, var: &str| {
        if v.len() != n {
            panic!(
                "the length of {}({}) and {}({}) differs",
                "value_date",
                n,
                &var,
                v.len()
            )
        }
    };
    check(&mty_date, "mty_date");
    check(&redem_value, "redem_value");
    check(&cpn_rate, "cpn_rate");
    check(&cpn_freq, "cpn_freq");
    check(&ref_date, "ref_date");
    check(&clean_price, "clean_price");

    let value_date = robj2date(value_date, "value_date");
    let mty_date = robj2date(mty_date, "mty_date");
    let ref_date = robj2date(ref_date, "ref_date");
    let redem_value = redem_value
        .as_real_slice()
        .expect("redem_value must be double");
    let cpn_rate = cpn_rate.as_real_slice().expect("cpn_rate must be double");
    let clean_price = clean_price
        .as_real_slice()
        .expect("clean_price must be double");
    let cpn_freq = cpn_freq.as_integer_slice().expect("cpn_freq must be int");

    let mut ytm: Vec<Option<f64>> = Vec::new();
    let mut macd: Vec<Option<f64>> = Vec::new();
    let mut modd: Vec<Option<f64>> = Vec::new();
    for i in 0..n {
        if value_date[i] == None
            || mty_date[i] == None
            || ref_date[i] == None
            || redem_value[i].is_na()
            || cpn_rate[i].is_na()
            || clean_price[i].is_na()
            || cpn_freq[i].is_na()
        {
            ytm.push(None);
            macd.push(None);
            modd.push(None);
        } else {
            let bond = bond::FixedBond::new(
                value_date[i].unwrap(),
                mty_date[i].unwrap(),
                redem_value[i],
                cpn_rate[i],
                cpn_freq[i] as u32,
            );
            let ref_date = ref_date[i].unwrap();
            match bond.result(&ref_date, clean_price[i]) {
                Some(out) => {
                    ytm.push(Some(out.ytm));
                    macd.push(Some(out.macd));
                    modd.push(Some(out.modd));
                }
                None => {
                    ytm.push(None);
                    macd.push(None);
                    modd.push(None);
                }
            }
        }
    }
    data_frame!(ytm = ytm, macd = macd, modd = modd)
}
// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod fcl;
    fn bond_result;
}
