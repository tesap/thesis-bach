#include <iostream>

int main() {
    int* int_ptr;
    int_ptr = new int(10);

    std::cout << int_ptr << "\n";

    delete int_ptr;
}
