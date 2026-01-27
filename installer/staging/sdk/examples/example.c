#include "meika256.h"
#include <stdio.h>

int main() {
    const char* msg = "Hello Meika";
    const char* password = "secret";

    uint8_t* encrypted = NULL;
    size_t encrypted_len = 0;

    if (meika_encrypt_buffer(
            (const uint8_t*)msg,
            strlen(msg),
            password,
            &encrypted,
            &encrypted_len) != MEIKA_OK) {
        printf("Encryption failed\n");
        return 1;
    }

    uint8_t* decrypted = NULL;
    size_t decrypted_len = 0;

    if (meika_decrypt_buffer(
            encrypted,
            encrypted_len,
            password,
            &decrypted,
            &decrypted_len) != MEIKA_OK) {
        printf("Decryption failed\n");
        return 1;
    }

    printf("Decrypted: %.*s\n", (int)decrypted_len, decrypted);

    meika_free(encrypted);
    meika_free(decrypted);

    return 0;
}
