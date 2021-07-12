#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void *smalloc(size_t size);

void sfree(void *p);

void *srealloc(void *p, size_t size);

char *dupstr(const char *s);

char *bin2hex(const unsigned char *input, int inlen);

unsigned char *hex2bin(const char *input, int outlen);
