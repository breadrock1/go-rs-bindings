#include <stdio.h>
#include "../rslib.h"

int main() {
    int add = addC(2, 2);
    int del = delC(4, 2);
    printf("Sum: %d\nDel: %d\n", add, del);
    return 0;
}
