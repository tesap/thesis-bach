
#include<iostream>

#include "single.cpp"
#include "../../1-object-as-resource-owner/cpp/file.cpp"

int main() {
    File* ptr = object_alloc_init<File>((char *) "file2.txt");

    std::cout << "== FD: " << ptr->fd << "\n";

    object_dealloc_deinit(ptr);
}
