#ifndef FFISHIM_TESTS_HELPERS
#define FFISHIM_TESTS_HELPERS

#include <stdio.h>
#include <string.h>

#define SEPARATOR printf("---\n");
#define TEST(call, type, todo) { \
	result_t *r = call; \
	if (r->message != NULL) { \
		printf("error: %s\n", r->message); \
	} else { \
		type res = *(type*)(r->payload); \
		todo; \
	} \
	free_result(r); \
}

#define HEAP_STRING(str) ({ \
	char *s = malloc(strlen(str) + 1); \
	strcpy(s, str); \
	s; \
})

#endif // FFISHIM_TESTS_HELPERS
