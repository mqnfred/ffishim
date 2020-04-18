#ifndef FFISHIM_TESTS_HELPERS
#define FFISHIM_TESTS_HELPERS

#include <stdio.h>

#define printo(prefix, format, return_type, call) { \
	outcome_t *outcome = call; \
	if (outcome == NULL) { \
		printf("outcome is NULL\n"); \
	} else if (outcome->errorcode != 0) { \
		printf(prefix "%ld %s\n", outcome->errorcode, outcome->message); \
	} else { \
		printf(prefix format "\n", *(return_type*)outcome->payload);\
	} \
}

#endif // FFISHIM_TESTS_HELPERS
