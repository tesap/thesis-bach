#include <stdio.h>
#include <iostream>


void doo(int* ptr) {
    int i0 = *ptr;
    printf("%d", i0);
    int i1 = *(ptr + 1);
    printf("%d", i1);
    int i2 = *(ptr + 2);
    int i22 = ptr[2];
    printf("%d", i2);
    printf("%d", i22);
}

int main() {
    int arr[3] = {10, 20, 30};
    int* ptr = arr;
    std::cout << arr[0] << arr[1] << arr[2];
    std::cout << ptr;

    doo(ptr);


    /*ptr++;*/
    /*printf("%d", ptr);*/
    /*ptr += 1;*/
    /*printf("%d", ptr);*/

}

