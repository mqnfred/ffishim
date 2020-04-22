#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "../../ffishim/header.h"
#include "../helpers.h"

typedef void *app_t;
extern void *free_app(app_t *app);
extern result_t *ffi_add_to_app_size(app_t *app, uint64_t additional);
extern result_t *ffi_get_app_size(app_t *app);
extern result_t *ffi_get_app();

int main() {
	TEST(ffi_get_app(), app_t*, {
		printf("%d\n", res != NULL);
		TEST(ffi_add_to_app_size(res, 9), app_t*, {
			printf("%d\n", res != NULL);
			TEST(ffi_get_app_size(res), uint64_t, {
				printf("%lu\n", res);
			});
		});
	});
	SEPARATOR;

	return 0;
}
