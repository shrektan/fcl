use chrono::{Datelike, NaiveDate};
use extendr_api::prelude::*;

// The days from 1970-1-1 (R's first date) to CE (1-1-0)
const R_DATE_FROM_CE: i32 = 719163;

pub fn robj2date(x: Robj, var: &str) -> Result<Vec<Option<NaiveDate>>> {
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

pub fn to_rdate(x: &Option<NaiveDate>) -> Option<f64> {
    match x {
        Some(v) => Some(date2rnum(v)),
        None => None,
    }
}

pub fn make_rdate(x: Vec<Option<f64>>) -> Robj {
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
