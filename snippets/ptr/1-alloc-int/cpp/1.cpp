
#include <stdio.h>
int main() {
    int n = 111111;
    int *ptr = &n;
    printf("%d", ptr);

    *ptr = 222222;
    printf("%d", ptr);
}
