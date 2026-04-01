use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::error::Error;

fn on_disk_sort(fs: (&str, &str)) -> Result<(), Box<dyn Error>> {
  let (unsorted_f, sorted_f) = fs;

  let f = File::open(unsorted_f)?;
  let mut lines = BufReader::new(f).lines();

  let first_line = lines
    .next()
    .ok_or("file is empty")??;
  let universe: usize = first_line
    .trim()
    .parse()?;
  let words = (universe + 63) / 64;

  let mut bitmap: Vec<u64> = vec![0u64; words];
  for line in lines {
    let line = line?;
    let num: usize = line.parse()?;
    if num >= universe {
      return Err("number greater than max".into());
    }

    let word = num / 64;
    let bit = num % 64;
    let mask = 1u64 << bit;
    if bitmap[word] & mask == 0 {
      bitmap[word] |= mask;
    } else {
      return Err("found duplicates!".into());
    }
  }

  let f_sorted = File::create(sorted_f)?;
  let mut writer = BufWriter::new(f_sorted);

  for (w, word_ref) in bitmap.iter_mut().enumerate() {
    let mut word = *word_ref;
    while word != 0 {
      let bit = word.trailing_zeros() as usize;
      let val = w * 64 + bit;
      if val < universe { writeln!(writer, "{}", val)?; }
      word &= word - 1;
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  fn check_sorted_file(file_name: &str) -> Result<bool, Box<dyn Error>> {
    let f = File::open(file_name)?;
    let mut lines = BufReader::new(f).lines();
    let _ = lines.next().ok_or("file is empty")?;
    let mut prev: usize = 0;
    for line in lines {
      let line = line?;
      let num: usize = line.trim().parse()?;
      if num < prev { return Ok(false); }
      prev = num;
    }

    Ok(true)
  }

  #[test]
  fn test_1() {
    let file_names = ("numbers.txt", "numbers_sorted.txt");
    on_disk_sort(file_names).unwrap();
    assert!(check_sorted_file(file_names.1).unwrap());

    let l = BufReader::new(File::open(file_names.1).unwrap())
      .lines()
      .count();
    assert_eq!(l, 40);
  }
}
