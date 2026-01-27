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
   Buffer mode
   ========================= */

/*
 * Encrypt memory buffer
 * Caller must free output using meika_free()
 */
int meika_encrypt_buffer(
    const uint8_t* data,
    size_t len,
    const char* password,
    uint8_t** out_ptr,
    size_t* out_len
);

/*
 * Decrypt memory buffer
 * Caller must free output using meika_free()
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

void meika_free(void* ptr);

#ifdef __cplusplus
}
#endif

#endif /* MEIKA256_H */
