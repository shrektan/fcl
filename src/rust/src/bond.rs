use chrono::{Datelike, DateTime, NaiveDate, Utc};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct FixedBond {
    value_date: NaiveDate,
    mty_date: NaiveDate,
    redem_value: f64,
    cpn_rate: f64,
    cpn_freq: u32,
}

#[derive(Debug)]
pub struct BondVal {
    pub ytm: f64,
    pub macd: f64,
    pub modd: f64,
}

#[derive(Debug)]
struct Cashflow {
    data: BTreeMap<NaiveDate, f64>,
}
impl Cashflow {
    fn size(&self) -> usize {
        self.data.len()
    }
    fn new() -> Cashflow {
        let data: BTreeMap<NaiveDate, f64> = BTreeMap::new();
        return Cashflow { data };
    }
    fn cf(&self, ref_date: &NaiveDate, price: f64) -> Cashflow {
        if self.size() == 0 {
            return Cashflow::new();
        }
        let mut data: BTreeMap<NaiveDate, f64> = BTreeMap::new();
        data.insert(*ref_date, -price);
        for (k, v) in &self.data {
            if k > ref_date {
                data.insert(*k, *v);
            }
        }
        Cashflow { data }
    }
    fn xirr_cf(&self) -> (Vec<DateTime<Utc>>, Vec<f64>) {
        let mut cfs: Vec<f64> = Vec::new();
        let mut dates: Vec<DateTime<Utc>> = Vec::new();
        for (k, v) in &self.data {
            cfs.push(*v);
            dates.push(DateTime::<Utc>::from_utc(k.and_hms(0, 0, 0), Utc));
        }
        (dates, cfs)
    }
}

impl FixedBond {
    pub fn new(value_date: NaiveDate, mty_date: NaiveDate, redem_value: f64, cpn_rate: f64, cpn_freq: u32) -> FixedBond {
        FixedBond {
            value_date, mty_date, redem_value, cpn_rate, cpn_freq
        }
    }
    fn years(d1: &NaiveDate, d0: &NaiveDate) -> f64 {
        (d1.year() - d0.year()) as f64
        // must be as f64 first, otherwise u32 - u32 may overflow (when negative)
            + (d1.month() as f64 - d0.month() as f64) / 12.0
            + (d1.day() as f64 - d0.day() as f64) / 365.0
    }
    fn add_months(ref_date: &NaiveDate, months: u32) -> NaiveDate {
        let num_of_months = ref_date.year() * 12 + ref_date.month() as i32 + months as i32;
        let year = num_of_months / 12;
        let month = num_of_months % 12;
        let since = NaiveDate::signed_duration_since;
        let max_day = since(
            NaiveDate::from_ymd(year, (month + 1) as u32, 1),
            NaiveDate::from_ymd(year, month as u32, 1),
        )
        .num_days() as u32;
        let day = ref_date.day();
        NaiveDate::from_ymd(
            year,
            month as u32,
            if day > max_day { max_day } else { day },
        )
    }
    fn cpn_dates(&self) -> Vec<NaiveDate> {
        let mut dates: Vec<NaiveDate> = vec![self.value_date];
        let mut ref_date = self.value_date;
        loop {
            match self.nxt_cpn_date(&ref_date) {
                Some(date) => {
                    ref_date = date;
                    dates.push(date);
                }
                None => break,
            }
        }
        dates
    }
    fn nxt_cpn_date(&self, ref_date: &NaiveDate) -> Option<NaiveDate> {
        if ref_date == &self.mty_date {
            return None;
        }
        let res = match self.cpn_freq {
            1 => Some(FixedBond::add_months(ref_date, 12)),
            2 => Some(FixedBond::add_months(ref_date, 6)),
            4 => Some(FixedBond::add_months(ref_date, 3)),
            12 => Some(FixedBond::add_months(ref_date, 1)),
            0 => Some(self.mty_date),
            other => panic!("unexpected cpn_freq {}", other),
        };
        match res {
            Some(date) => {
                if date > self.mty_date {
                    None
                } else {
                    Some(date)
                }
            }
            None => None,
        }
    }
    fn cpn_value(&self) -> f64 {
        let factor = match self.cpn_freq {
            1 => 1.0,
            2 => 0.5,
            4 => 0.25,
            12 => 1.0 / 12.0,
            0 => FixedBond::years(&self.mty_date, &self.value_date),
            other => panic!("unexpected cpn_freq {}", other),
        };
        self.redem_value * self.cpn_rate * factor
    }
    fn accrued(&self, ref_date: &NaiveDate) -> f64 {
        if ref_date >= &self.mty_date || ref_date <= &self.value_date {
            return 0.0;
        }
        let cpn_dates = self.cpn_dates();
        match cpn_dates.binary_search(&ref_date) {
            Ok(_) => 0.0, // when ok, it means it's one of the cpn date and the coupon has been paid then should be zero
            Err(i) => {
                // dbg!(&cpn_dates); dbg!(&ref_date); dbg!(i);
                let last_cpn_date = cpn_dates[i - 1];
                let nxt_cpn_date = cpn_dates[i];
                let cpn_days = nxt_cpn_date.signed_duration_since(last_cpn_date).num_days();
                let days = ref_date.signed_duration_since(last_cpn_date).num_days();
                // dbg!(cpn_days); dbg!(days);
                self.cpn_value() / cpn_days as f64 * days as f64
            }
        }
    }
    fn dirty_price(&self, ref_date: &NaiveDate, clean_price: f64) -> f64 {
        clean_price + self.accrued(ref_date)
    }
    fn cashflow(&self) -> Cashflow {
        let mut ref_date = self.nxt_cpn_date(&self.value_date);
        let mut res: Cashflow = Cashflow::new();
        loop {
            match ref_date {
                Some(date) => {
                    let value: f64 = if date == self.mty_date {
                        self.redem_value
                    } else {
                        0.0
                    } + self.cpn_value();
                    res.data.insert(date, value);
                    ref_date = self.nxt_cpn_date(&date);
                }
                None => break,
            }
        }
        res
    }
    pub fn result(&self, ref_date: &NaiveDate, clean_price: f64) -> BondVal {
        let dirty_price = self.dirty_price(ref_date, clean_price);
        let cf = self.cashflow().cf(ref_date, dirty_price).xirr_cf();
        let ytm = financial::xirr(&cf.1, &cf.0, None).unwrap();
        let ytm_chg = 1e-6;
        let npv1 = financial::xnpv(ytm + ytm_chg, &cf.1, &cf.0).unwrap();
        let npv0 = financial::xnpv(ytm - ytm_chg, &cf.1, &cf.0).unwrap();
        let modd = -(npv1 - npv0) / (2.0 * ytm_chg * dirty_price);
        let cf2 = self.cashflow().cf(ref_date, dirty_price);
        let years : Vec<f64> = cf2.data.keys().map(|date : &NaiveDate| {
            FixedBond::years(date, ref_date)
        }).collect();
        let macd = &years.iter().zip(&cf.1).map(|(t, cf)| {
            cf * t * (1.0 + ytm).powf(-t)
        }).sum() / dirty_price;
        BondVal {
            ytm, macd, modd
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn round(x: f64, digit: Option<u32>) -> f64 {
        let digit = digit.unwrap_or(4);
        let scale: f64 = 10f64.powf(digit as f64);
        (x * scale).round() / scale
    }
    fn rnd(x: f64) -> f64 {
        round(x, Some(3))
    }
    fn rnd2(x: f64) -> f64 {
        round(x, Some(2))
    }
    #[test]
    fn dirty_price() {
        let mut bond = FixedBond::new(
            NaiveDate::from_ymd(2010, 1, 1),
            NaiveDate::from_ymd(2015, 1, 1),
            100.0,
            0.05,
            2,
        );
        let ref_date = NaiveDate::from_ymd(2010, 1, 1);
        assert_eq!(bond.accrued(&ref_date), 0.0);
        let ref_date = NaiveDate::from_ymd(2011, 7, 1);
        assert_eq!(bond.dirty_price(&ref_date, 100.0), 100.0);
        let ref_date = NaiveDate::from_ymd(2011, 1, 1);
        assert_eq!(bond.dirty_price(&ref_date, 100.0), 100.0);

        bond.cpn_freq = 1;
        let ref_date = NaiveDate::from_ymd(2010, 2, 1);
        assert_eq!(bond.accrued(&ref_date), 31.0 / 365.0 * 5.0);

        let bond = FixedBond {
            value_date: NaiveDate::from_ymd(2010, 1, 1),
            mty_date: NaiveDate::from_ymd(2012, 1, 1),
            redem_value: 100.0,
            cpn_rate: 0.05,
            cpn_freq: 0,
        };
        let ref_date = NaiveDate::from_ymd(2010, 2, 1);
        assert_eq!(
            bond.accrued(&ref_date),
            31.0 / (365.0 + 365.0) * (5.0 * 2.0)
        );
    }
    #[test]
    fn plain_bond() {
        let bond = FixedBond {
            value_date: NaiveDate::from_ymd(2010, 1, 1),
            mty_date: NaiveDate::from_ymd(2020, 1, 1),
            redem_value: 100.0,
            cpn_rate: 0.05,
            cpn_freq: 1,
        };
        let ytm = 0.05;
        let ref_date = NaiveDate::from_ymd(2010, 1, 1);
        assert_eq!(rnd(bond.result(&ref_date, 100.0).ytm), ytm);
        // won't change as the price is clean
        let ref_date = NaiveDate::from_ymd(2011, 1, 1);
        assert_eq!(rnd(bond.result(&ref_date, 100.0).ytm), ytm);
        // won't change as the price is clean
        let ref_date = NaiveDate::from_ymd(2011, 6, 15);
        assert_eq!(rnd(bond.result(&ref_date, 100.0).ytm), ytm);
    }
    #[test]
    fn zero_cpn_bond() {
        let bond = FixedBond {
            value_date: NaiveDate::from_ymd(2010, 1, 1),
            mty_date: NaiveDate::from_ymd(2011, 1, 1),
            redem_value: 100.0,
            cpn_rate: 0.05,
            cpn_freq: 0,
        };
        let ytm = 0.050000000000000114;
        let ref_date = NaiveDate::from_ymd(2010, 1, 1);
        assert_eq!(bond.result(&ref_date, 100.0).ytm, ytm);
    }
    #[test]
    fn dur() {
        let bond = FixedBond {
            value_date: NaiveDate::from_ymd(2010, 1, 1),
            mty_date: NaiveDate::from_ymd(2015, 1, 1),
            redem_value: 100.0,
            cpn_rate: 0.05,
            cpn_freq: 0,
        };
        let ref_date = NaiveDate::from_ymd(2010, 1, 1);
        assert_eq!(rnd2(bond.result(&ref_date, 100.0).macd), 5.0);
        let ref_date = NaiveDate::from_ymd(2011, 1, 1);
        assert_eq!(rnd2(bond.result(&ref_date, 100.0).macd), 4.0);
        let ref_date = NaiveDate::from_ymd(2010, 7, 1);
        assert_eq!(rnd2(bond.result(&ref_date, 100.0).macd), 4.5);

        let bond = FixedBond {
            value_date: NaiveDate::from_ymd(2010, 1, 1),
            mty_date: NaiveDate::from_ymd(2015, 1, 1),
            redem_value: 100.0,
            cpn_rate: 0.05,
            cpn_freq: 1,
        };
        let ref_date = NaiveDate::from_ymd(2010, 1, 1);
        let res = bond.result(&ref_date, 100.0);
        assert_eq!(rnd2(res.macd / (1.0 + res.ytm)), rnd2(res.modd));
    }
    #[test]
    #[should_panic]
    fn panic_when_invalid_freq() {
        let bond = FixedBond {
            value_date: NaiveDate::from_ymd(2010, 1, 1),
            mty_date: NaiveDate::from_ymd(2011, 1, 1),
            redem_value: 100.0,
            cpn_rate: 0.05,
            cpn_freq: 3,
        };
        bond.cashflow();
    }
}
