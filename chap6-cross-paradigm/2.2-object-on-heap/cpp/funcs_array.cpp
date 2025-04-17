// #include <cstdlib>
#include <stdlib.h>

// using namespace std;

/*template <typename T>*/
/*T* array_alloc_init(size_t size) {*/
/*    return new T[size];*/
/*}*/
/**/
/*template <typename T>*/
/*void array_dealloc_deinit(T* ptr) {*/
/*    delete[] ptr;*/
/*}*/


template <typename T>
T* array_alloc(size_t size) {
    return (T*) malloc(sizeof(T) * size);
}

template <typename T>
void array_dealloc(T* ptr, size_t size) {
    free(ptr);
}

template <typename T, typename... Args>
void array_init(T* ptr, size_t index, Args&&... args) {
    ptr[index] = std::move(T{args...}); // TODO is it best practice?
}

template <typename T>
void array_deinit(T* ptr, size_t size) {
    for (int i = 0; i < size; i++) {
        ptr[i].~T(); // TODO Is it?
    }
}
