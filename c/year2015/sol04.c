#include <openssl/evp.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

const char *SECRET_KEY = "ckczppom";       // puzzle input
const char *TEST_SECRET_KEY1 = "abcdef";   // example key 1
const char *TEST_SECRET_KEY2 = "pqrstuv";  // example key 2

// md5_digest computes the MD5 digest for the given message. The returned
// pointer should be passed to free(3) to release the allocated storage when it
// is no longer needed. This function return a NULL pointer if an error occurs.
char *md5_digest(const char *message) {
  EVP_MD_CTX *mdctx = EVP_MD_CTX_new();
  const EVP_MD *md = EVP_md5();       // digest type - md5
  uint8_t md_value[EVP_MAX_MD_SIZE];  // digest value
  uint32_t md_len;                    // digest length

  EVP_DigestInit(mdctx, md);
  EVP_DigestUpdate(mdctx, message, strlen(message));
  EVP_DigestFinal(mdctx, md_value, &md_len);
  EVP_MD_CTX_free(mdctx);

  char *buf = calloc(2 * md_len + 1, sizeof(char));
  if (buf == NULL) {
    return NULL;
  }
  for (uint32_t i = 0; i < md_len; i++) {
    snprintf(&(buf[i * 2]), 3, "%02x", md_value[i]);  // 2 hex chars + '\0'
  }
  return buf;
}

// itoa will convert the given integer value i to ASCII string. The returned
// pointer should be passed to free(3) to release the allocated storage when it
// is no longer needed. This function return a NULL pointer if an error occurs.
char *itoa(int i) {
  ssize_t len = snprintf(NULL, 0, "%i", i);
  if (len == -1) {
    goto err;
  }
  char *buf = malloc(len * sizeof(char) + 1);
  if (buf == NULL) {
    goto err;
  }
  if (snprintf(buf, len + 1, "%d", i) == -1) {
    goto err;
  }
  return buf;

err:
  return NULL;
}

int is_number_valid(const char *key, int num, int zeroes) {
  char *numstr = itoa(num);
  if (numstr == NULL) {
    return -1;
  }
  size_t dest_size = strlen(numstr) + strlen(key) + 1;
  char *message = malloc(dest_size);
  strlcat(message, key, dest_size);
  strlcat(message, numstr, dest_size);

  char *digest = md5_digest(message);
  int i;
  for (i = 0; i < zeroes; i++) {
    if (digest[i] != '0') {
      break;
    }
  }

  free(numstr);
  free(digest);
  return i == zeroes;
}

int year2015_sol04() {
  int i;
  for (i = 0;; i++) {
    if (is_number_valid(SECRET_KEY, i, 5)) {
      break;
    }
  }
  printf("4.1: %d\n", i);

  for (i = 0;; i++) {
    if (is_number_valid(SECRET_KEY, i, 6)) {
      break;
    }
  }
  printf("4.2: %d\n", i);

  return EXIT_SUCCESS;
}
