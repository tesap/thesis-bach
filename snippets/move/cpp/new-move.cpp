



#include <iostream>

int main() {
  int a = 10;

  int& b = a;

  int c = b;

  std::cout << "C: " << c << "\n";
}
