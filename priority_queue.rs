#[derive(Debug, Clone)]
pub struct PriorityQueue<T> {
  pub data: Vec<T>,
}

impl<T: Ord + Clone> PriorityQueue<T> {
  pub fn from_vec(data: Vec<T>) -> Self {
    let len = data.len();
    let mut heap = Self { data };

    for i in (0..len / 2).rev() {
      heap.sift_down(i);
    }

    heap
  }

  pub fn heap_sort(&mut self) -> Vec<T> {
    let mut bheap = self.clone();
    let mut result = Vec::new();

    loop {
      let val = bheap.pop();
      if let Some(v) = val {
        result.push(v);
      } else {
        break;
      }
    }

    result.reverse();
    result
  }

  pub fn pop(&mut self) -> Option<T> {
    let last = self.data.pop()?;

    if self.data.is_empty() {
      return Some(last);
    }

    let result = std::mem::replace(&mut self.data[0], last);
    self.sift_down(0);
    Some(result)
  }

  fn sift_down(&mut self, i: usize) {
    let len = self.data.len();
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    let mut largest = i;

    if left < len && self.data[left] > self.data[largest] {
      largest = left;
    }

    if right < len && self.data[right] > self.data[largest] {
      largest = right;
    }

    if largest != i {
      self.data.swap(i, largest);
      self.sift_down(largest);
    }
  }

  /*
  fn sift_down(&mut self, mut i: usize) {
    let len = self.data.len();

    loop {
      let left = 2 * i + 1;
      let right = 2 * i + 2;
      let mut largest = i;

      if left < len && self.data[left] > self.data[largest] {
        largest = left;
      }

      if right < len && self.data[right] > self.data[largest] {
        largest = right;
      }

      if largest == i {
        break;
      }

      self.data.swap(i, largest);
      i = largest;
    }
  }
  */
}

#[cfg(test)]
mod tests {
  use super::*;

  fn is_max_heap<T: Ord>(data: &[T]) -> bool {
    let len = data.len();

    for i in 0..len / 2 {
      let left = 2 * i + 1;
      let right = 2 * i + 2;

      if left < len && data[i] < data[left] {
        return false;
      }

      if right < len && data[i] < data[right] {
        return false;
      }
    }

    true
  }

  #[test]
  fn test_bheap_sift() {
    let bheap = PriorityQueue::from_vec(vec![3, 5, 1, 29, 3, 59, 6, 9, 2]);
    assert!(!bheap.data.is_empty());
    assert!(is_max_heap(&bheap.data));
  }

  #[test]
  fn test_bheap_pop() {
    let mut bheap = PriorityQueue::from_vec(vec![34, 5, 46, 1, 23, 8, 34, 69, 67]);
    assert!(!bheap.data.is_empty());
    assert!(is_max_heap(&bheap.data));
    let popped = bheap.pop();
    assert_eq!(popped, Some(69));
    assert!(is_max_heap(&bheap.data));
  }

  #[test]
  fn test_heap_sort() {
    let mut bheap = PriorityQueue::from_vec(vec![34, 5, 46, 1, 23, 8, 34, 69, 67]);
    assert!(!bheap.data.is_empty());
    assert!(is_max_heap(&bheap.data));
    let sorted = bheap.heap_sort();
    for i in 0..sorted.len() - 1 {
      if sorted[i] > sorted[i + 1] {
        assert!(false);
      }
    }
  }
}
