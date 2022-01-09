pub fn assert_near_eq(left: &Vec<Option<f64>>, right: &Vec<Option<f64>>) {
  let mut failed = false;
  for (i, left_val) in left.iter().enumerate() {
      match (left_val, right[i]) {
          (Some(l), Some(r)) => {
              if (l - r).abs() > f64::EPSILON.sqrt() {
                  failed = true;
                  break;
              }
          },
          (None, None) => {

          },
          _ => {
              failed = true;
              break;
          },
      }
  }
  if failed {
      panic!("'assert near equal failed: `(left == right)`\n left: {:?}\nright: {:?}", left, right);
  }
}
