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
extern gps_t *new_gps(double lat, double lon);
extern void free_gps(gps_t *gps);
extern outcome_t *ffi_push_gps(array_t *coordinates, gps_t *gps);

typedef struct player {
	char *name;
	array_t *points;
} player_t;
extern player_t *new_player(char *name, array_t *points);
extern void free_player(player_t *p);
extern outcome_t *ffi_player_sum_points(player_t *p);

int main() {
	array_t *points = ARRAY_NEW(uint64_t, 5);
	ARRAY_PUSH(uint64_t, points, 126);
	ARRAY_PUSH(uint64_t, points, 721);
	ARRAY_PUSH(uint64_t, points, 2379);
	player_t *p1 = new_player(HEAP_STRING("player1"), points);
	TEST(ffi_player_sum_points(p1), uint64_t, {
		printf("%ld\n", res);
	});
	SEPARATOR;

	array_t *coordinates = ARRAY_NEW(gps_t*, 5);
	ARRAY_PUSH(gps_t*, coordinates, new_gps(3.3, 4.4));
	ARRAY_PUSH(gps_t*, coordinates, new_gps(5.5, 6.6));
	printf("len=%lu cap=%lu\n", coordinates->len, coordinates->cap);
	TEST(ffi_push_gps(coordinates, new_gps(7.7, 8.8)), array_t*, {
		printf("len=%lu cap=%lu\n", res->len, res->cap);
		for (int i = 0; i < res->len; i++) {
			printf(
				"%lf %lf\n",
				ARRAY_GET(gps_t*, res, i)->lat,
				ARRAY_GET(gps_t*, res, i)->lon
			);
		}
		ARRAY_FREE(gps_t*, res, free_gps);
	});
	SEPARATOR;

	return 0;
}
