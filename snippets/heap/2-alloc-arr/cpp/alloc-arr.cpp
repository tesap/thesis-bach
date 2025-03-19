
#include <stdlib.h>
#include <iostream>


void print_arr(int *arr, int size) {
    std::cout << "[";
    for (int i = 0; i < size - 1; i++) {
        std::cout << arr[i] << ", ";
    }
    std::cout << arr[size - 1] << "]\n";
}

int main() {
    const int size = 1000;
    int* ptr = (int *) malloc(sizeof(int) * size);

    *ptr = 222222;

    print_arr(ptr, size);
}
