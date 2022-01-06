use chrono::{NaiveDate};
use extendr_api::prelude::*;
mod bond;
mod rdate;

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

    let value_date = rdate::robj2date(value_date, "value_date").unwrap();
    let mty_date = rdate::robj2date(mty_date, "mty_date").unwrap();
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
        rdate::to_rdate(&Some(*v))
    }).collect();
    data_frame!(ID = ids, DATE = rdate::make_rdate(rdates), CF = cfs)
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
    let ref_date = rdate::robj2date(ref_date, "ref_date").unwrap();
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
