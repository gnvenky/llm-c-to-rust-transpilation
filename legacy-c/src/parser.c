#include "parser.h"
#include <string.h>

int parse_message(Buffer *buf, Message *msg) {
    if (!buf || !msg || buf->len < sizeof(Message)) return -1;

    msg->id = buf->data[0];
    msg->value = (buf->data[1] << 8) | buf->data[2];
    return 0;
}
// legacy C parser
