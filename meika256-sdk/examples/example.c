#include <stdio.h>
#include <string.h>
#include "meika256.h"

int main(void) {
    const char* message = "Hello Meika";
    const char* password = "pass123";

    uint8_t* encrypted = NULL;
    size_t encrypted_len = 0;

    uint8_t* decrypted = NULL;
    size_t decrypted_len = 0;

    if (meika_encrypt_buffer(
            (const uint8_t*)message,
            strlen(message),
            password,
            &encrypted,
            &encrypted_len) != MEIKA_OK) {
        printf("Encryption failed\n");
        return 1;
    }

    if (meika_decrypt_buffer(
            encrypted,
            encrypted_len,
            password,
            &decrypted,
            &decrypted_len) != MEIKA_OK) {
        printf("Decryption failed\n");
        meika_free(encrypted);
        return 1;
    }

    printf("Decrypted: %.*s\n", (int)decrypted_len, decrypted);

    meika_free(encrypted);
    meika_free(decrypted);

    return 0;
}
