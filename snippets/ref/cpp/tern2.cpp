#include <iostream>

int& get_lvalue(int& a, int& b, bool condition) {
  return condition ? a : b;
}

int main() {
  int x = 1;
  int y = 2;

  // Modify x or y through the returned reference
  get_lvalue(x, y, true) = 10;  // Modifies x
  get_lvalue(x, y, false) = 20; // Modifies y

  std::cout << "x: " << x << ", y: " << y << std::endl; // Output: x: 10, y: 20
}
