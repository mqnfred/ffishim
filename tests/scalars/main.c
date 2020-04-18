#include <stdio.h>
#include <stdint.h>
#include "../../ffishim/src/library/header.h"
#include "../helpers.h"

extern outcome_t* ffi_add(int64_t a, int64_t b);

int main() {
	outcome_t *add_result = ffi_add(1, 3);
	printf("ffi_add(1, 3) = %ld\n", *((int64_t*)(add_result->payload)));
	return 0;
}
