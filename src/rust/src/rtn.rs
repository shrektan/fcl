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
    fn dates(from: RDate, to: RDate) -> Result<Vec<RDate>, String> {
        if from >= to {
            return Err("from should be smaller than to".to_string());
        }
        Ok((from..=to).collect())
    }
    fn i_dates(&self, from: RDate, to: RDate) -> Result<Vec<usize>, String> {
        let i_from = self.i(from).ok_or("from is out range")?;
        let i_to = self.i(to).ok_or("to is out range")?;
        if i_from >= i_to {
            return Err("from should be smaller than to".to_string());
        }
        Ok((i_from..=i_to).collect())
    }
    fn twrr_dr(&self, from: RDate, to: RDate) -> Result<Vec<Option<f64>>, String> {
        let i_dates = self.i_dates(from, to)?;
        let drs = i_dates.iter().map(|i| self.dr(*i)).collect();
        Ok(drs)
    }
    fn twrr_cr(&self, from: RDate, to: RDate) -> Result<Vec<Option<f64>>, String> {
        let mut out = self.twrr_dr(from, to)?;
        out = Self::crs(&out);
        Ok(out)
    }
    fn weighted_cf(i_dates: &Vec<usize>, cfs: &[f64], i: usize) -> f64 {
        let i_dates = i_dates.get(0..=i).unwrap();
        let total_days = i_dates.last().unwrap() - i_dates.first().unwrap();
        let weights: Vec<f64> = i_dates
            .iter()
            .map(|i| (i_dates.last().unwrap() - i) as f64 / total_days as f64)
            .collect();
        let weighted_cf: f64 = cfs.iter().zip(weights).map(|(cf, wt)| cf * wt).sum();
        weighted_cf
    }
    fn dietz_avc(&self, from: RDate, to: RDate) -> Result<Vec<f64>, String> {
        let i_dates = self.i_dates(from, to)?;
        let mv0: f64 = *self.mv0(i_dates[0]).ok_or("can't fetch mv0")?;
        let cfs: Vec<f64> = i_dates.iter().map(|i| self.cf(*i).unwrap_or(0.0)).collect();
        let out: Vec<f64> = i_dates
            .iter()
            .enumerate()
            .map(|(i, _)| Self::weighted_cf(&i_dates, &cfs, i) + mv0)
            .collect();
        Ok(out)
    }
    fn dietz(&self, from: RDate, to: RDate) -> Result<Vec<f64>, String> {
        let i_dates = self.i_dates(from, to)?;
        let pls: Vec<f64> = i_dates.iter().map(|i| *self.pl(*i).unwrap()).collect();
        let mut cum_pls: Vec<f64> = Vec::with_capacity(pls.len());
        for (i, pl) in pls.iter().enumerate() {
            if i == 0 {
                cum_pls.push(*pl);
            } else {
                cum_pls.push(cum_pls[i - 1] + pl);
            }
        }
        let avcs: Vec<f64> = self.dietz_avc(from, to)?;
        let out: Vec<f64> = cum_pls.iter().zip(avcs).map(|(pl, avc)| pl / avc).collect();
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

        let twrr_dates = Rtn::dates(1, 5).unwrap();
        let twrr_cr = rtn.twrr_cr(1, 5).unwrap();
        assert_eq!(twrr_dates, vec![1, 2, 3, 4, 5]);
        assert_eq!(twrr_cr, vec![None, None, None, None, None]);
        assert_near_eq!(rtn.mvs, vec![100., 100., 102., 103., 104.]);
        assert_near_eq!(rtn.pls, vec![0., 0., 2., 1., 1.]);

        let twrr_dates = Rtn::dates(2, 5).unwrap();
        let twrr_dr = rtn.twrr_dr(2, 5).unwrap();
        assert_eq!(twrr_dates, vec![2, 3, 4, 5]);
        assert_near_eq!(
            twrr_dr,
            vec![Some(0.0), Some(0.02), Some(1. / 102.), Some(1. / 103.)]
        );
        let twrr_cr = rtn.twrr_cr(2, 5).unwrap();
        assert_near_eq!(twrr_cr, vec![Some(0.0), Some(0.02), Some(0.03), Some(0.04)]);
    }
    #[test]
    fn dietz_ok() {
        let dates = vec![1, 2, 3, 4];
        let mvs = vec![100., 102., 103., 104.];
        let pls = vec![0., 2., 1., 1.];
        let rtn = Rtn::new(dates, mvs, pls).unwrap();
        let avc = rtn.dietz_avc(2, 4).unwrap();
        let dietz = rtn.dietz(2, 4).unwrap();
        assert_near_eq!(avc, vec![100., 100., 100.]);
        assert_near_eq!(dietz, vec![0.02, 0.03, 0.04]);
    }
}
