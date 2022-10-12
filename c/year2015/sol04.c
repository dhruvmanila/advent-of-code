#include <openssl/evp.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const char *secret_key = "ckczppom";  // puzzle input
// static const char *test_secret_key1 = "abcdef";   // example key 1
// static const char *test_secret_key2 = "pqrstuv";  // example key 2

int year2015_sol04(void) {
  int num1 = 0, num2 = 0;
  int retval = EXIT_FAILURE;
  char message[32];
  size_t len;

  EVP_MD_CTX *mdctx = EVP_MD_CTX_new();  // initialize the context
  if (mdctx == NULL) {
    perror("EVP_MD_CTX_new");
    goto free_ctx_and_return;
  }
  const EVP_MD *md = EVP_md5();     // digest type - md5
  uint8_t digest[EVP_MAX_MD_SIZE];  // digest value

  for (int i = 0; !num1 || !num2; i++) {
    len = snprintf(message, sizeof(message), "%s%d", secret_key, i);
    if (len < 1 || len >= sizeof(message)) {  // len does not include '\0'
      perror("snprintf");
      goto free_ctx_and_return;
    }

    EVP_DigestInit(mdctx, md);
    EVP_DigestUpdate(mdctx, message, len);
    if (!EVP_DigestFinal(mdctx, digest, NULL)) {
      perror("EVP_DigestFinal");
      goto free_ctx_and_return;
    }

    if (!num1 && !digest[0] && !digest[1] && !(digest[2] & 0xF0)) {
      num1 = i;
    }
    if (!num2 && !digest[0] && !digest[1] && !digest[2]) {
      num2 = i;
    }
  }

  printf("4.1: %d\n4.2: %d\n", num1, num2);
  retval = EXIT_SUCCESS;

free_ctx_and_return:
  if (mdctx != NULL) {
    EVP_MD_CTX_free(mdctx);
  }
  return retval;
}
