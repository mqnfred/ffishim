#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "../../ffishim/header.h"
#include "../helpers.h"

extern result_t *ffi_give_option();
extern result_t *ffi_take_option(int32_t *opt);
extern result_t *ffi_string_option(char **s);
extern result_t *ffi_option_result(int32_t *opt);

int main() {
	TEST(ffi_give_option(), int32_t*, {
		printf("%d\n", *res);
		free(res);
	});
	SEPARATOR;

	TEST(ffi_take_option(SOME(int32_t, 5)), int32_t*, printf("%p\n", res));
	TEST(ffi_take_option(NONE), int32_t*, {
		printf("%d\n", *res);
		free(res);
	});
	SEPARATOR;

	TEST(ffi_string_option(NONE), char**, printf("%p\n", res));
	TEST(ffi_string_option(SOME(char*, HEAP_STRING("ffi"))), char**, {
		printf("%s\n", *res);
		free(*res);
		free(res);
	});
	SEPARATOR;

	TEST(ffi_option_result(SOME(int32_t, 5)), int32_t*, {
		printf("%d\n", *res);
		free(res);
	});
	TEST(ffi_option_result(NONE), int32_t*, printf("%p\n", res));

	return 0;
}
