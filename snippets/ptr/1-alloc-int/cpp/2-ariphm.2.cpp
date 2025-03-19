#include <stdio.h>
#include <iostream>

int main() {
  int arr[3] = {10, 20, 30};
  int* ptr = arr;

  // --- Addition
  int* ptr_add = ptr + 2;
  std::cout << ptr_add;

  // --- Derefernce
  int i0 = *ptr;
  std::cout << i0;

  // --- Combined: Addition + Dereference
  int i2 = ptr[2];
  // OR
  // int i2 = *(ptr + 2);
  std::cout << i2;
}
