#include<iostream>

#include "../../1-object-as-resource-owner/cpp/file.cpp"

int main() {
    File f1("file1.txt");

    File f_copy = f1; // Implicit copy constructor
    File f_move = std::move(f1); // Move constructor

    // Use moved object
    std::cout << f_move.fd;
}
