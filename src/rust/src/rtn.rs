use std::collections::BTreeMap;

type RDate = i32;

struct Rtn {
    dates: Vec<RDate>,
    mvs: Vec<f64>,
    pls: Vec<f64>,
}

impl Rtn {
    fn new(dates: Vec<RDate>, mvs: Vec<f64>, pls: Vec<f64>) -> Result<Self, String> {
        let n = dates.len();
        if mvs.len() != n {
            return Err("the len of mvs and dates doesn't equal".to_string());
        }
        if pls.len() != n {
            return Err("the len of pls and dates doesn't equal".to_string());
        }
        let mut data: BTreeMap<RDate, (f64, f64)> = BTreeMap::new();
        for (i, date) in dates.iter().enumerate() {
            if data.contains_key(&date) {
                return Err("dates contain duplicate".to_string());
            }
            data.insert(*date, (mvs[i], pls[i]));
        }
        let min_date = *data.keys().min().unwrap();
        let max_date = *data.keys().max().unwrap();
        let dates: Vec<RDate> = (min_date..=max_date).collect();
        let keys: Vec<RDate> = data.keys().cloned().collect();
        let mut mvs: Vec<f64> = Vec::new();
        let mut pls: Vec<f64> = Vec::new();
        for d in dates.iter() {
            match keys.binary_search(d) {
                Ok(i) => {
                    let v = data.get(&keys[i]).unwrap();
                    mvs.push(v.0);
                    pls.push(v.1);
                }
                Err(i) => {
                    let v = data.get(&keys[i - 1]).unwrap();
                    mvs.push(v.0);
                    pls.push(0.0);
                }
            };
        }
        Ok(Self { dates, mvs, pls })
    }
    fn mv(&self, i: usize) -> Option<&f64> {
        self.mvs.get(i)
    }
    fn mv0(&self, i: usize) -> Option<&f64> {
        if i == 0 {
            None
        } else {
            self.mv(i - 1)
        }
    }
    fn pl(&self, i: usize) -> Option<&f64> {
        self.pls.get(i)
    }
    fn cf(&self, i: usize) -> Option<f64> {
        Some(self.mv(i)? - self.mv0(i)? - self.pl(i)?)
    }
    fn dr(&self, i: usize) -> Option<f64> {
        let cf = self.cf(i)?;
        let deno = if cf >= 0.0 { cf } else { 0.0 };
        Some(self.pl(i)? / (self.mv0(i)? + deno))
    }
    fn crs(drs: &Vec<Option<f64>>) -> Vec<Option<f64>> {
        let mut out: Vec<Option<f64>> = Vec::with_capacity(drs.len());
        for (i, dr) in drs.iter().enumerate() {
            let v = if i == 0 {
                *dr
            } else {
                if dr.is_none() || out[i - 1].is_none() {
                    None
                } else {
                    Some((out[i - 1].unwrap() + 1.) * (dr.unwrap() + 1.) - 1.)
                }
            };
            out.push(v);
        }
        out
    }
    fn i(&self, date: RDate) -> Option<usize> {
        match self.dates.binary_search(&date) {
            Ok(k) => Some(k),
            Err(_) => None,
        }
    }
    fn twrr_dr(&self, from: RDate, to: RDate) -> Result<(Vec<RDate>, Vec<Option<f64>>), String> {
        let i_from = self.i(from).ok_or("from is out range")?;
        let i_to = self.i(to).ok_or("to is out range")?;
        let i_dates: Vec<usize> = (i_from..=i_to).collect();
        let drs = i_dates.iter().map(|i| self.dr(*i)).collect();
        let dates: Vec<RDate> = (from..=to).collect();
        Ok((dates, drs))
    }
    fn twrr_cr(&self, from: RDate, to: RDate) -> Result<(Vec<RDate>, Vec<Option<f64>>), String> {
        let mut out = self.twrr_dr(from, to)?;
        out.1 = Self::crs(&out.1);
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert::NearEq;
    #[test]
    fn twrr_work() {
        let dates = vec![1, 3, 4, 5];
        let mvs = vec![100., 102., 103., 104.];
        let pls = vec![0., 2., 1., 1.];
        let rtn = Rtn::new(dates, mvs, pls).unwrap();

        let twrr_cr = rtn.twrr_cr(1, 5).unwrap();
        assert_eq!(twrr_cr.0, vec![1, 2, 3, 4, 5]);
        assert_eq!(twrr_cr.1, vec![None, None, None, None, None]);
        assert_eq!(rtn.mvs, vec![100., 100., 102., 103., 104.]);
        assert_eq!(&rtn.pls, &vec![0., 0., 2., 1., 1.]);

        let twrr_dr = rtn.twrr_dr(2, 5).unwrap();
        assert_eq!(twrr_dr.0, vec![2, 3, 4, 5]);
        assert_eq!(
            twrr_dr.1,
            vec![Some(0.0), Some(0.02), Some(1. / 102.), Some(1. / 103.)]
        );
        let twrr_cr = rtn.twrr_cr(2, 5).unwrap();
        assert_eq!(twrr_cr.0, vec![2, 3, 4, 5]);
        Vec::<Option<f64>>::assert_near_eq(
            &twrr_cr.1,
            &vec![Some(0.0), Some(0.02), Some(0.03), Some(0.04)],
        );
    }
}
