#include <stdio.h>
#include <arpa/inet.h>
#include <stdint.h>


int main() {
    uint16_t i = 0x1f40;

    char* p = (char*) &i;

    for (int i = 0; i < 2; i++) {
        printf("0x%02x ", *p);
        p++;
    };
    printf("\n");

    uint16_t ni = htons(i);

    p = (char*) &ni;

    for (int i = 0; i < 2; i++) {
        printf("0x%02x ", *p);
        p++;
    };
    printf("\n");

}