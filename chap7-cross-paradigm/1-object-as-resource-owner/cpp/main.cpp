
#include<iostream>
#include "file.cpp"

int main() {
    File f1("file1.txt");
    std::cout << f1.fd << "\n";

    File f_copy = f1; // Implicit copy constructor
    File f_move = std::move(f1); // Move constructor

    // Use moved object
    std::cout << f_move.fd << "\n";
}
