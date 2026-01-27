#ifndef MEIKA256_H
#define MEIKA256_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* =========================
   Error codes
   ========================= */
#define MEIKA_OK               0
#define MEIKA_INVALID_INPUT    1
#define MEIKA_CRYPTO_ERROR     2
#define MEIKA_IO_ERROR         3

/* =========================
   Buffer mode (memory)
   ========================= */

/*
 * Encrypt a memory buffer.
 * 
 * data     : input buffer
 * len      : input length
 * password : null-terminated UTF-8 string
 * out_ptr  : allocated output buffer (must be freed with meika_free)
 * out_len  : output length
 */
int meika_encrypt_buffer(
    const uint8_t* data,
    size_t len,
    const char* password,
    uint8_t** out_ptr,
    size_t* out_len
);

/*
 * Decrypt a memory buffer.
 */
int meika_decrypt_buffer(
    const uint8_t* data,
    size_t len,
    const char* password,
    uint8_t** out_ptr,
    size_t* out_len
);

/* =========================
   File mode (streaming)
   ========================= */

int meika_encrypt_file(
    const char* input_path,
    const char* output_path,
    const char* password
);

int meika_decrypt_file(
    const char* input_path,
    const char* output_path,
    const char* password
);

/* =========================
   Memory management
   ========================= */

void meika_free(uint8_t* ptr);

#ifdef __cplusplus
}
#endif

#endif /* MEIKA256_H */
