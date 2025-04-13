
#include<iostream>
#include "single.cpp"
#include "../../../chap4-memory-management/cpp/data.cpp"

int main() {
    Data* p = object_alloc_init<Data>(1, 2, 3);

    p->x1 = 10;
    std::cout << p->x1 << ", " << p->x2 << ", " << p->x3 << "\n";

    object_dealloc_deinit(p);
}
