#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "../../ffishim/header.h"
#include "../helpers.h"

extern outcome_t *ffi_add(int64_t a, int64_t b);

int main() {
	outcome_t *outcome_1 = ffi_add(1, 3);
	PRINTO("ffi_add(1, 3) = ", "%ld", int64_t, outcome_1);
	free_outcome(outcome_1);
	SEPARATOR;

	return 0;
}
