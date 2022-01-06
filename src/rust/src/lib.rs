use chrono::{Datelike, NaiveDate};
use extendr_api::prelude::*;
mod bond;

// The days from 1970-1-1 (R's first date) to CE (1-1-0)
const R_DATE_FROM_CE: i32 = 719163;

fn robj2date(x: Robj, var: &str) -> Result<Vec<Option<NaiveDate>>> {
    if !x.inherits("Date") {
        return Result::Err( Error::Other(format!("{} is not a Date", var)) );
    }
    let out = match x.rtype() {
        RType::Real => x
            .as_real_iter()
            .unwrap()
            .map(|d| {
                if d.is_na() {
                    None
                } else {
                    NaiveDate::from_num_days_from_ce_opt(d as i32 + R_DATE_FROM_CE)
                }
            })
            .collect(),
        RType::Integer => x
            .as_integer_iter()
            .unwrap()
            .map(|d| {
                if d.is_na() {
                    None
                } else {
                    NaiveDate::from_num_days_from_ce_opt(d + R_DATE_FROM_CE)
                }
            })
            .collect(),
        _ => {
            return Result::Err( Error::Other(format!("{} is Date but the type is not integer or double", var)) );
        }
    };
    Result::Ok(out)
}

fn date2rnum(x: &NaiveDate) -> f64 {
    (x.num_days_from_ce() - R_DATE_FROM_CE) as f64
}

fn to_rdate(x: &Option<NaiveDate>) -> Option<f64> {
    match x {
        Some(v) => Some(date2rnum(v)),
        None => None,
    }
}

fn make_rdate(x: Vec<Option<f64>>) -> Robj {
    r!(x).set_class(&["Date"]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_date() {
        test! {
            single_threaded(|| {
                let x: Robj = r!([18990.0, 18991.0]).set_class(&["Date"]).unwrap();
                assert_eq!(robj2date(x, "x").unwrap(), [Some(NaiveDate::from_ymd(2021, 12, 29)), Some(NaiveDate::from_ymd(2021, 12, 30))]);
            });
        }
    }
}

fn check_len(x: [&Robj; 2], var: [&str; 2]) {
    if x[0].len() != x[1].len() {
        panic!(
            "the length of {}({}) and {}({}) differs",
            var[0],
            x[0].len(),
            var[1],
            x[1].len()
        )
    }
}

fn make_bond(
    value_date: Robj,
    mty_date: Robj,
    redem_value: Robj,
    cpn_rate: Robj,
    cpn_freq: Robj,
) -> Vec<Option<bond::FixedBond>> {
    let n = value_date.len();
    check_len([&value_date, &mty_date], ["value_date", "mty_date"]);
    check_len([&value_date, &redem_value], ["value_date", "redem_value"]);
    check_len([&value_date, &cpn_rate], ["value_date", "cpn_rate"]);
    check_len([&value_date, &cpn_freq], ["value_date", "cpn_freq"]);

    let value_date = robj2date(value_date, "value_date").unwrap();
    let mty_date = robj2date(mty_date, "mty_date").unwrap();
    let redem_value = redem_value
        .as_real_slice()
        .expect("redem_value must be double");
    let cpn_rate = cpn_rate.as_real_slice().expect("cpn_rate must be double");

    let cpn_freq = cpn_freq.as_integer_slice().expect("cpn_freq must be int");
    let mut out: Vec<Option<bond::FixedBond>> = Vec::new();
    for i in 0..n {
        if value_date[i] == None
            || mty_date[i] == None
            || redem_value[i].is_na()
            || cpn_rate[i].is_na()
            || cpn_freq[i].is_na()
        {
            out.push(None);
        } else {
            let bond = bond::FixedBond::new(
                value_date[i].unwrap(),
                mty_date[i].unwrap(),
                redem_value[i],
                cpn_rate[i],
                cpn_freq[i],
            );
            if bond.is_ok() {
                out.push(Some(bond.unwrap()));
            } else {
                out.push(None);
            }
        }
    }
    out
}


/// Generate bond's cash flows
/// @inheritParams bond_result
/// @export
#[extendr]
fn bond_cf(
    value_date: Robj,
    mty_date: Robj,
    redem_value: Robj,
    cpn_rate: Robj,
    cpn_freq: Robj
) -> Robj {
    let bonds = make_bond(
        value_date, mty_date, redem_value, cpn_rate, cpn_freq,
    );
    let mut ids: Vec<i32> = Vec::new();
    let mut dates: Vec<NaiveDate> = Vec::new();
    let mut cfs: Vec<f64> = Vec::new();
    for (i, bond) in bonds.iter().enumerate() {
        match bond {
            Some(value) => {
                let cf = value.cashflow();
                cfs.append(&mut cf.values());
                dates.append(&mut cf.dates());
                ids.append(&mut vec![i as i32 + 1; cf.len()]);
            },
            None => {
            }
        }
    }
    let rdates: Vec<Option<f64>> = dates.iter().map(|v| {
        to_rdate(&Some(*v))
    }).collect();
    data_frame!(ID = ids, DATE = make_rdate(rdates), CF = cfs)
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
    check_len([&value_date, &ref_date], ["value_date", "ref_date"]);
    check_len([&value_date, &clean_price], ["value_date", "clean_price"]);
    let ref_date = robj2date(ref_date, "ref_date").unwrap();
    let clean_price = clean_price
        .as_real_slice()
        .expect("clean_price must be double");
    let bonds = make_bond(
        value_date, mty_date, redem_value, cpn_rate, cpn_freq,
    );
    struct Out {
        ytm: Vec<Option<f64>>,
        macd: Vec<Option<f64>>,
        modd: Vec<Option<f64>>,
    }
    impl Out {
        fn new() -> Self {
            Out {
                ytm: Vec::new(),
                macd: Vec::new(),
                modd: Vec::new(),
            }
        }
        fn push_none(&mut self) {
            self.ytm.push(None);
            self.macd.push(None);
            self.modd.push(None);
        }
        fn push(&mut self, value: bond::BondVal) {
            self.ytm.push(Some(value.ytm));
            self.macd.push(Some(value.macd));
            self.modd.push(Some(value.modd));
        }
    }
    let mut out = Out::new();
    for (i, bond) in bonds.iter().enumerate() {
        if ref_date[i].is_none() || clean_price[i].is_na() {
            out.push_none();
            continue;
        }
        match bond {
            Some(bond) => {
                let ref_date = ref_date[i].unwrap();
                match bond.result(&ref_date, clean_price[i]) {
                    Some(value) => {
                        out.push(value);
                    }
                    None => out.push_none()
                }
            },
            None => out.push_none(),
        }
    }
    data_frame!(ytm = out.ytm, macd = out.macd, modd = out.modd)
}
// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod fcl;
    fn bond_result;
    fn bond_cf;
}
