fn rot(s: &str, k: usize) -> String {
  let n = s.len();
  if n == 0 { return "".to_string(); }
  let k = k % n;
  String::from(
    s[n-k..n].to_string() +
    &s[0..n-k].to_string()
  )
}

fn rot_inplace(s: &mut String, k: usize) {
  let n = s.len();
  if n == 0 { return; }
  let k = k % n;
  let bytes = unsafe { s.as_bytes_mut() };
  bytes.rotate_right(k);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let out = rot("ABCDEF", 4);
    assert_eq!(out, "CDEFAB");

    let out = rot("ABCDEF", 0);
    assert_eq!(out, "ABCDEF");

    let out = rot("ABCDEF", 8);
    assert_eq!(out, "EFABCD");

    let out = rot("ABCDEF", 6);
    assert_eq!(out, "ABCDEF");

    let out = rot("", 6);
    assert_eq!(out, "");
  }

  #[test]
  fn test_inplace() {
    let mut s = String::from("ABCDEF");
    rot_inplace(&mut s, 4);
    assert_eq!(s, "CDEFAB");

    s = String::from("ABCDEF");
    rot_inplace(&mut s, 0);
    assert_eq!(s, "ABCDEF");

    s = String::from("ABCDEF");
    rot_inplace(&mut s, 8);
    assert_eq!(s, "EFABCD");

    s = String::from("ABCDEF");
    rot_inplace(&mut s, 6);
    assert_eq!(s, "ABCDEF");

    s = String::from("");
    rot_inplace(&mut s, 6);
    assert_eq!(s, "");
  }
}
