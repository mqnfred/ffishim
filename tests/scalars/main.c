#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "../../ffishim/header.h"
#include "../helpers.h"

extern outcome_t *ffi_add_i64(int64_t a, int64_t b);
extern outcome_t *ffi_add_f64(double a, float b);
extern outcome_t *ffi_fails(int64_t a);

int main() {
	outcome_t *outcome_1 = ffi_add_i64(1, 3);
	PRINTO("ffi_add_i64(1, 3) = ", "%ld", int64_t, outcome_1);
	free_outcome(outcome_1);

	outcome_t *outcome_2 = ffi_add_i64(-51, 3);
	PRINTO("ffi_add_i64(-51, 3) = ", "%ld", int64_t, outcome_2);
	free_outcome(outcome_2);

	SEPARATOR;

	outcome_t *outcome_3 = ffi_add_f64(1.2, 3.5);
	PRINTO("ffi_add_f64(1.2, 3.5) = ", "%lf", double, outcome_3);
	free_outcome(outcome_3);

	outcome_t *outcome_4 = ffi_add_f64(-1.2, 3.5);
	PRINTO("ffi_add_f64(-1.2, 3.5) = ", "%lf", double, outcome_4);
	free_outcome(outcome_4);

	SEPARATOR;

	outcome_t *outcome_5 = ffi_fails(5);
	PRINTO("ffi_fails(5) = ", "%ld", int64_t, outcome_5);
	free_outcome(outcome_5);

	outcome_t *outcome_6 = ffi_fails(4);
	PRINTO("ffi_fails(4) = ", "%ld", int64_t, outcome_6);
	free_outcome(outcome_6);

	return 0;
}
