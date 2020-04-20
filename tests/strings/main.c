#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "../../ffishim/header.h"
#include "../helpers.h"

extern outcome_t *ffi_give_string();
extern outcome_t *ffi_take_string(const char *s);

int main() {
	TEST(ffi_give_string(), char*, {
		printf("%s\n", res);
		free(res);
	});

	TEST(ffi_take_string(HEAP_STRING("ffi")), char*, {
		printf("%s\n", res);
		free(res);
	});

	return 0;
}
