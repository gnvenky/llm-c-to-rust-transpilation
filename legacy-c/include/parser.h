#ifndef PARSER_H
#define PARSER_H

#include <stdint.h>
#include <stddef.h>

typedef struct {
    uint8_t *data;
    size_t len;
} Buffer;

typedef struct {
    int id;
    uint16_t value;
} Message;

int parse_message(Buffer *buf, Message *msg);

#endif // PARSER_H
// legacy C header
