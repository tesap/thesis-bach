
struct Data {
    // Constructor
    Data(int x1, int x2, int x3) : x1{x1}, x2{x2}, x3{x3} {};
    Data(const Data&& other): x1{other.x1}, x2{other.x2}, x3{other.x3} {};

    int x1;
    int x2;
    int x3;
};
