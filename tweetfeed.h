#pragma once

#include <stdint.h>

struct RustByteSlice {
    const uint8_t *bytes;
    size_t length;
};

int32_t rust_print(char*);
void* twitter_create(void);
void twitter_destroy(void*);
void* tweet_iter_create(void*)
void tweet_iter_destroy(void*)
void* tweet_iter_next(void*)
struct RustByteSlice tweet_get_username(*void)
void tweet_destroy(*void)
