#ifndef FFISHIM_TESTS_HELPERS
#define FFISHIM_TESTS_HELPERS

#include <stdio.h>

#define PRINTO(prefix, format, return_type, call) { \
	outcome_t *outcome = call; \
	if (outcome == NULL) { \
		printf("outcome is NULL\n"); \
	} else if (outcome->message != NULL) { \
		printf(prefix "%s\n", outcome->message); \
	} else { \
		printf(prefix format "\n", *(return_type*)outcome->payload);\
	} \
}

#define SEPARATOR printf("---\n");

void free_outcome(outcome_t *outcome) {
	if (outcome->message != NULL) {
		free(outcome->message);
	} else {
		free(outcome->payload);
	}
	free(outcome);
}

#endif // FFISHIM_TESTS_HELPERS
