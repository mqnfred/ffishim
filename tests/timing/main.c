#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "../../ffishim/header.h"
#include "../helpers.h"

extern result_t *ffi_get_duration();

int main() {
	TEST(ffi_get_duration(), uint64_t, printf("%ld\n", res));
	SEPARATOR;

	return 0;
}
