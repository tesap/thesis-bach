
#include <iostream>

class MyClass {
public:
    MyClass() { std::cout << "Constructor\n"; }
    MyClass(const MyClass&) { std::cout << "Copy Constructor\n"; }
    MyClass(MyClass&&) { std::cout << "Move Constructor\n"; }
    ~MyClass() { std::cout << "Destructor\n"; }
};

MyClass createObject(bool flag) {
    std::cout << "== 2\n";
    MyClass obj1;
    std::cout << "== 3\n";
    MyClass obj2;
    std::cout << "== 4\n";
    if (flag) {
        std::cout << "== 5\n";
        return obj1; // NRVO may apply here
    } else {
        return obj2; // NRVO may apply here
    }
}

int main() {
    std::cout << "0\n";
    MyClass result = createObject(true);
    std::cout << "1\n";
    return 0;
}
