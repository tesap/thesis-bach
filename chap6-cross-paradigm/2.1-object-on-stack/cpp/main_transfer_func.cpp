#include<iostream>

#include "../../1-object-as-resource-owner/cpp/file.cpp"

void foo(File& f) {
    std::cout << "=== lvalue ref.\n";
    // Copy
    File f_copy = f;
}

void foo(File&& f) {
    std::cout << "=== rvalue ref.\n";
    // Move
    File f_move = f;
}

template <typename T>
void general(T&& ref) {
    foo(std::forward<T>(ref));
}

int main() {
    File f1("file1.txt");
    general(f1); // Copy
    general(std::move(f1)); // Move
}
