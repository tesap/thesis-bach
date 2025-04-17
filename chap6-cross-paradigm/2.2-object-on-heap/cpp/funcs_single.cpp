
template <typename T, typename... Args>
T* object_alloc_init(Args&&... args) {
    return new T(std::forward<Args>(args)...);
}

template <typename T>
void object_dealloc_deinit(T* ptr) {
    delete ptr;
}
