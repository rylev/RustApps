#pragma once

#include <stdint.h>

struct RustByteSlice {
    const uint8_t *bytes;
    size_t length;
};

int32_t rust_print(char*);
void* twitter_create(void);
void twitter_destroy(void*);
void* tweet_list_create(void*);
void tweet_list_destroy(void*);
void* tweet_list_get(void*, size_t);
size_t tweet_list_len(void*);
struct RustByteSlice tweet_get_username(void*);
struct RustByteSlice tweet_get_text(void*);
