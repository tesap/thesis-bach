
#include<iostream>


void val(int n) {
  n++;
}

void r_ref(int&& n) {
  n++;
}

void l_ref(int& n) {
  n++;
  std::cout << n << "\n";
}

int main() {
    int a = 0;
    /*(int&)*/a = 10;
    std::cout << a << "\n";

    int b = /*(int)*/a;
    int& aref = (int&) a;

    l_ref(/*(int&)*/ a);
    std::cout << a << "\n";

    l_ref(b);

    const int x = 10;
    const int& x_ref = x;
    const int* x2 = &aref;

    val(aref);
    val(x_ref);
}
