#ifndef FFISHIM_LIBRARY
#define FFISHIM_LIBRARY

typedef struct outcome {
	int64_t errorcode;
	const char *message;
	const void *payload;
} outcome_t;

#endif // FFISHIM_LIBRARY
