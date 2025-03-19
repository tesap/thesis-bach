#include <iostream>
#include <cassert>

int main() {
    // reinterpret_cast
    int number = 42;
    int* ptr = &number;

    char* ptr_char = reinterpret_cast<char*>(ptr);
    long int* ptr_long = reinterpret_cast<long int*>(ptr);

    // const_cast
    const int constant = 100;
    int* mut_ptr = const_cast<int*>(&constant);

    *mut_ptr = 200;
    assert(constant == 200);
}
