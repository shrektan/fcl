#![macro_use]

pub trait NearEq {
    fn near_equal(&self, right: &Self) -> bool;
}

impl NearEq for Vec<Option<f64>> {
    fn near_equal(&self, right: &Self) -> bool {
        let left = &self;
        let mut failed = false;
        for (i, left_val) in left.iter().enumerate() {
            match (left_val, right[i]) {
                (Some(l), Some(r)) => {
                    if (l - r).abs() > f64::EPSILON.sqrt() {
                        failed = true;
                        break;
                    }
                }
                (None, None) => {}
                _ => {
                    failed = true;
                    break;
                }
            }
        }
        !failed
    }
}

impl NearEq for Vec<f64> {
    fn near_equal(&self, right: &Self) -> bool {
        let left = &self;
        let mut failed = false;
        for (i, left_val) in left.iter().enumerate() {
            if (left_val - right[i]).abs() > f64::EPSILON.sqrt() {
                failed = true;
                break;
            }
        }
        !failed
    }
}

#[macro_export]
macro_rules! assert_near_eq {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val.near_equal(right_val)) {
                    panic!(
                        "'assert near equal failed: `(left == right)`\n left: {:?}\nright: {:?}",
                        *left_val, *right_val
                    );
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ok() {
    let x: Vec<f64>= vec![1., 2., 3.];
    let y: Vec<f64> = vec![1., 2., 3.];
    assert_near_eq!(x, y);
  }

  #[test]
  #[should_panic(expected = "assert near equal failed")]
  fn test_fail() {
    let x: Vec<f64>= vec![1., 2., 3.];
    let y: Vec<f64> = vec![1., 2., 4.];
    assert_near_eq!(x, y);
  }
}
