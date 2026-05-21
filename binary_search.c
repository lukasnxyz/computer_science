#include <stdio.h>

int binary_search(const int vals[], size_t len, int target) {
  int left = 0, right = len;

  while (left <= right) {
    size_t mid = left + (right - left) / 2;

    if (vals[mid] == target)
      return mid;
    else if (vals[mid] < target)
      left = mid + 1;
    else
      right = mid - 1;
  }

  return -1;
}

#define LEN 11

int main(void) {
  const int vals[LEN] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
  const int mval = 9;

  const int out_index = binary_search(vals, LEN, mval);

  printf("%d == %d\n", mval, out_index);

  return 0;
}
