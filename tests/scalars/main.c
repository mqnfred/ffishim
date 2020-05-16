#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "../../ffishim/header.h"
#include "../helpers.h"

extern result_t *ffi_add_i64(int64_t a, int64_t b);
extern result_t *ffi_add_f64(double a, float b);
extern result_t *ffi_fails(int64_t a);
extern result_t *ffi_not(int8_t a);

int main() {
	TEST(ffi_add_i64(1, 3), int64_t, printf("%ld\n", res));
	TEST(ffi_add_i64(-51, 3), int64_t, printf("%ld\n", res));
	SEPARATOR;

	TEST(ffi_add_f64(1.2, 3.5), double, printf("%lf\n", res));
	TEST(ffi_add_f64(-1.2, 3.5), double, printf("%lf\n", res));
	SEPARATOR;

	TEST(ffi_fails(5), int64_t, printf("%ld\n", res));
	TEST(ffi_fails(4), int64_t, printf("%ld\n", res));
	SEPARATOR;

	TEST(ffi_not(1), int8_t, printf("%d\n", res));
	TEST(ffi_not(0), int8_t, printf("%d\n", res));
	SEPARATOR;

	return 0;
}
