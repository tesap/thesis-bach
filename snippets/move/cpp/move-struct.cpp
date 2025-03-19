#include <iostream>


class A {
public:
  A(int a1, int a2, int a3)
    : a1{a1}
    , a2{a2}
    , a3{a3}
  {
    std::cout << "Constructor\n";
  };

  A(const A& a) {
    std::cout << "Copy constructor\n";
  }

  A(const A&& a) {
    std::cout << "Move constructor\n";
  }

  int a1;
  int a2;
  int a3;
};

// Expects a reference that is live oustide
void foo_ref(A& aa) {
  aa.a1++;
}

// Expects a reference to temporary object that is not live oustide
void foo_rvalue_ref(A&& aa) {
  aa.a1++;
}

int main() {
  /*A x(1, 2, 3);*/
  /**/
  /*std::cout << "X.a1: " << x.a1 << "\n";*/
  /**/
  /*A y = x; // Copy*/
  /*y.a1 = 10;*/
  /**/
  /*std::cout << "Y.a1: " << y.a1 << "\n";*/
  /*std::cout << "X.a1: " << x.a1 << "\n";*/
  /**/
  /*A y2 = std::move(x);*/

  // Rvalue ref
  A&& a1{1, 2, 3};

  foo_ref(a1);
  std::cout << a1.a1 << "\n";

  foo_rvalue_ref((A&&) a1);
  std::cout << a1.a1 << "\n";

  // Lvalue ref
  A a2(1, 2, 3);
  A& a2_ref = a2;

  foo_ref(a2_ref);
  std::cout << a2.a1 << "\n";

  foo_rvalue_ref((A&&) a2_ref);
  std::cout << a2.a1 << "\n";

  foo_rvalue_ref(std::move(a2_ref));
  std::cout << a2.a1 << "\n";
  std::cout << a2_ref.a1 << "\n";
}
