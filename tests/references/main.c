#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "../../ffishim/header.h"
#include "../helpers.h"

typedef void gps_t;
extern result_t *ffi_new_gps();
extern void free_gps(gps_t *gps);
extern result_t *ffi_set_lat(gps_t *gps, double lat);
extern result_t *ffi_get_lon(gps_t *gps);

int main() {
	TEST(ffi_new_gps(), gps_t*, {
		TEST(ffi_set_lat(res, 5.0), double, {
			printf("%lf\n", res);
		})
		TEST(ffi_get_lon(res), double, {
			printf("%lf\n", res);
		})
		free_gps(res);
	})
	SEPARATOR;

	return 0;
}
