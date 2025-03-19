#include <iostream>

int main() {
    // cannot take the address of an rvalue of type 'int'
    // int *ptr = &111111;

    int n = 111111;
    int *ptr = &n;
    std::cout << "PTR: " << ptr << "\n";

    *ptr = 222222;
    std::cout << "PTR: " << ptr << "\n";
}
