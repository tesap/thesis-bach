#include <iostream>

int main() {
    // cannot take the address of an rvalue of type 'int'
    // int *ptr = &111111;

    int *ptr;
    ptr = new int;
    std::cout << ptr;
}
