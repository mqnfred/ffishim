#ifndef FFISHIM_LIBRARY
#define FFISHIM_LIBRARY

typedef struct outcome {
	char *message;
	void *payload;
} outcome_t;

extern void free_outcome(outcome_t *outcome);

#define NONE NULL
#define SOME(type, val) ({ \
	type *tmp = malloc(sizeof(type)); \
	*tmp = (type)val; \
	tmp; \
})

#endif // FFISHIM_LIBRARY
