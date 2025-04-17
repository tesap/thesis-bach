
#include<iostream>

#include "funcs_array.cpp"
#include "../../1-object-as-resource-owner/cpp/file.cpp"

int main() {
    // File* ptr = object_alloc_init<File>((char *) "file2.txt");
    size_t size = 5;
    File* ptr = array_alloc<File>(size);

    // Manual initialization
    array_init(ptr, 0, (char *) "file1.txt");
    array_init(ptr, 1, (char *) "file2.txt");
    array_init(ptr, 2, (char *) "file3.txt");

    // std::cout << "== FD: " << ptr->fd << "\n";

    array_deinit(ptr, 3);
    array_dealloc(ptr, size);
}
