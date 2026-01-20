#include "parser.h"
#include <stdio.h>

int main() {
    uint8_t data[] = {1, 0x00, 0x2A};
    Buffer buf = { data, sizeof(data) };
    Message msg;
    
    int r = parse_message(&buf, &msg);
    if (r == 0) {
        printf("Parsed id=%d, value=%d\n", msg.id, msg.value);
        return 0;
    } else {
        printf("Parse failed\n");
        return 1;
    }
}

