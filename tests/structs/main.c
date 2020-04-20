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
extern void free_gps(gps_t *gps);

typedef struct player {
	char *name;
	uint64_t *age;
	gps_t *coordinates;
} player_t;
extern void free_player(player_t *p);

extern outcome_t *ffi_get_player_1();

int main() {
	TEST(ffi_get_player_1(), player_t*, {
		printf("%s\n", res->name);
		printf("%ld\n", *(res->age));
		printf("%lf\n", res->coordinates->lat);
		printf("%lf\n", res->coordinates->lon);
		free_player(res);
	});
	SEPARATOR;

	return 0;
}
