#ifndef FFISHIM_TESTS_HELPERS
#define FFISHIM_TESTS_HELPERS

#include <stdio.h>
#include <string.h>

#define SEPARATOR printf("---\n");
#define TEST(call, type, todo) { \
	outcome_t *o = call; \
	if (o->message != NULL) { \
		printf("error: %s\n", o->message); \
	} else { \
		type res = *(type*)(o->payload); \
		todo; \
	} \
	free_outcome(o); \
}

#define HEAP_STRING(str) ({ \
	char *s = malloc(strlen(str) + 1); \
	strcpy(s, str); \
	s; \
})

#endif // FFISHIM_TESTS_HELPERS
