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
extern outcome_t *ffi_cheese_shop(gps_t gps);

int main() {
	gps_t gps = { .lat = 37.405263, .lon = 2.0, .planet = NULL };
	outcome_t *outcome_7 = ffi_cheese_shop(gps);
	printf("ffi_cheese_shop(gps).lat = %lf\n", ((gps_t*)outcome_7->payload)->lat);
	printf("ffi_cheese_shop(gps).lon = %lf\n", ((gps_t*)outcome_7->payload)->lon);
	printf("ffi_cheese_shop(gps).planet = %p\n", ((gps_t*)outcome_7->payload)->planet);
	free_outcome(outcome_7);

	SEPARATOR;

	return 0;
}
