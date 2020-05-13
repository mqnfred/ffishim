#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "../../ffishim/header.h"
#include "../helpers.h"

extern result_t *ffi_add(int64_t a, int64_t b);

int main() {
	TEST(ffi_add(1, 3), int64_t, printf("%ld\n", res));
	SEPARATOR;

	return 0;
}
