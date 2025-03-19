
int main() {
    int x = 1;
    double y = 2.0;

    // left-hand is of type int&
    (x < y ? x : y) = 10;

}
