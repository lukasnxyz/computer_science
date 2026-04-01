use std::cmp::Ordering;

fn bsearch<T: Ord>(vals: &[T], target: &T) -> Option<usize> {
  let mut left = 0;
  let mut right = vals.len();

  while left <= right {
    let mid = left + (right - left) / 2;
    match vals[mid].cmp(target) {
      Ordering::Equal => return Some(mid),
      Ordering::Less => left = mid + 1,
      Ordering::Greater => right = mid - 1,
    }
  }

  return None;
}

fn bsearch_f<T: Ord>(
  vals: &[T],
  target: &T,
  left: usize,
  right: usize
) -> Option<usize> {
  if left > right {
    None
  } else {
    let mid = left + (right - left) / 2;
    match vals[mid].cmp(target) {
      Ordering::Equal =>
        Some(mid),
      Ordering::Less =>
        bsearch_f(vals, target, mid+1, right),
      Ordering::Greater =>
        bsearch_f(vals, target, left, mid-1),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let vals = &[1,2,3,4,5,8,9];
    let pos = bsearch(vals, &8);
    assert_eq!(pos, Some(5));

    let vals = &[1,2,3,4,5,8,9];
    let pos = bsearch(vals, &1);
    assert_eq!(pos, Some(0));

    let vals = &[1,2,3,4,5,8,9];
    let pos = bsearch(vals, &9);
    assert_eq!(pos, Some(6));
  }

  #[test]
  fn test_functional_version() {
    let vals = &[1,2,3,4,5,8,9];
    let pos = bsearch_f(vals, &8, 0, vals.len());
    assert_eq!(pos, Some(5));

    let vals = &[48,59,67,69,73,99,105];
    let pos = bsearch_f(vals, &48, 0, vals.len());
    assert_eq!(pos, Some(0));

    let vals = &[48,59,67,69,73,99,105];
    let pos = bsearch_f(vals, &105, 0, vals.len());
    assert_eq!(pos, Some(6));
  }
}
