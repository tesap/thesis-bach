#include <fcntl.h>
#include <unistd.h>


class File {
public:
    int fd{-1};

    explicit File(char* filename) {
        std::cout << "--- Constructor\n";
        fd = open(filename, O_RDWR | O_CREAT | O_TRUNC, 0644);
    }

    // Destructor
    ~File() {
        reset();
    }


    // Copy constructor
    File(const File& other) {
        std::cout << "--- Copy constructor\n";
        fd = dup(other.fd);
    }

    // Move constructor
    File(File&& other) {
        std::cout << "--- Move constructor\n";
        fd = other.fd;

        // Invalidate the moved-from without cleanup
        other.fd = -1;
    }

    File& operator=(File&& other) noexcept {
        std::cout << "--- Move assignment op.\n";
        if (this != &other) {
            // ~File();
            reset();

            // Steal the fd
            fd = other.fd;

            // Invalidate the moved-from object
            other.fd = -1;
        }
        return *this;
    }


    /*File& operator=(File&& other) noexcept {*/
    /*    std::cout << "--- Copy assignment op.\n";*/
    /*    if (this != &other) {*/
    /*        // ~File();*/
    /*        reset();*/
    /*    }*/
    /*    return *this;*/
    /*}*/
    /**/
private:
    void reset() {
        std::cout << "RESET\n";
        if (fd > 0) {
            std::cout << "FD: " << fd << "\n";
            close(fd);
            fd = -1;
        }
    }
};
