#include <stdio.h>
#include <stdint.h>
#include "../../ffishim/src/library/header.h"
#include "../helpers.h"

extern outcome_t *ffi_add_i64(int64_t a, int64_t b);
extern outcome_t *ffi_add_f64(double a, float b);

int main() {
	printo("ffi_add_i64(1, 3) = ", "%ld", int64_t, ffi_add_i64(1, 3));
	printo("ffi_add_f64(1.2, 3.5) = ", "%lf", double, ffi_add_f64(1.2, 3.5));
	return 0;
}
