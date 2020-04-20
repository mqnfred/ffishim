#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "../../ffishim/header.h"
#include "../helpers.h"

typedef struct gps {
	double lat;
	double lon;
	int64_t *planet;
} gps_t;
extern outcome_t *ffi_milk_pail_coordinates();

int main() {
	TEST(ffi_milk_pail_coordinates(), gps_t*, {
		printf("%lf\n", res->lat);
		printf("%lf\n", res->lon);
		free(res);
	});
	SEPARATOR;

	return 0;
}
