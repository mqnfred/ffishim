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

typedef struct player {
	char *name;
	uint64_t *age;
	gps_t *coordinates;
} player_t;
extern player_t *new_player(char *name, uint64_t *age, gps_t *coordinates);
extern void free_player(player_t *p);
extern result_t *ffi_set_player_lat(player_t *p, double lat);
extern result_t *ffi_get_player_1();

typedef struct coordinates {
	int64_t zero;
	int64_t one;
} coordinates_t;
extern coordinates_t *new_coordinates(int64_t zero, int64_t one);
extern void free_coordinates(coordinates_t *c);
extern result_t *ffi_set_x(coordinates_t *c, int64_t zero);

int main() {
	TEST(ffi_get_player_1(), player_t*, {
		printf("%s\n", res->name);
		printf("%ld\n", *(res->age));
		printf("%lf\n", res->coordinates->lat);
		printf("%lf\n", res->coordinates->lon);
		free_player(res);
	});
	SEPARATOR;

	player_t *p1 = new_player(
		HEAP_STRING("mny"),
		SOME(uint64_t, 28),
		new_gps(39.9, 28.8)
	);
	TEST(ffi_set_player_lat(p1, 40.0), player_t*, {
		printf("%s\n", res->name);
		printf("%ld\n", *(res->age));
		printf("%lf\n", res->coordinates->lat);
		printf("%lf\n", res->coordinates->lon);
		free_player(res);
	});
	SEPARATOR;

	coordinates_t *c1 = new_coordinates(1, 2);
	TEST(ffi_set_x(c1, 3), coordinates_t*, {
		printf("%ld\n", res->zero);
		printf("%ld\n", res->one);
		free_coordinates(res);
	});
	SEPARATOR;

	return 0;
}
