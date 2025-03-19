
#include <iostream>

template <typename T>
void black_box(T& value) {
    volatile T* p = &value;
    (void)p; // Prevent unused variable warning
}

class A {
public:
  A(int x1, int x2, int x3)
    : x1{x1}
    , x2{x2}
    , x3{x3}
  {};

  int x1;
  int x2;
  int x3;
};


void as_val(A in) {
  in.x1 += 100;
  in.x2 += 100;
  in.x3 += 100;
  black_box(in.x1);
  black_box(in.x2);
  black_box(in.x3);
  // std::cout << "AS_VAL: " << in.x1 << ", " << in.x2 << ", " << in.x3 << "\n";
}

void as_ptr(A* in) {
  in->x1 += 100;
  in->x2 += 100;
  in->x3 += 100;
  black_box(in->x1);
  black_box(in->x2);
  black_box(in->x3);
  // std::cout << "AS_PTR: " << in->x1 << ", " << in->x2 << ", " << in->x3 << "\n";
}

void as_ref(A& in) {
  in.x1 += 300;
  in.x2 += 300;
  in.x3 += 300;
  black_box(in.x1);
  black_box(in.x2);
  black_box(in.x3);
  // std::cout << "AS_REF: " << in.x1 << ", " << in.x2 << ", " << in.x3 << "\n";
}

int main() {
  A a1(10, 20, 30);

  as_val(a1);
  as_ptr(&a1);
  as_ref(a1);
  black_box(a1);
  black_box(a1.x1);
  black_box(a1.x2);
  black_box(a1.x3);
}
