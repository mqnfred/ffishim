#ifndef FFISHIM_LIBRARY
#define FFISHIM_LIBRARY

typedef struct result {
	char *message;
	void *payload;
} result_t;

extern void free_result(result_t *result);

#define NONE NULL
#define SOME(type, val) ({ \
	type *tmp = malloc(sizeof(type)); \
	*tmp = (type)val; \
	tmp; \
})

typedef struct array {
	void *ptr;
	size_t len;
	size_t cap;
} array_t;

#define ARRAY_NEW(type, capacity) ({ \
	array_t *arr = malloc(sizeof(array_t)); \
	arr->ptr = malloc(sizeof(void*) * capacity); \
	arr->len = 0; \
	arr->cap = capacity; \
	arr; \
})
#define ARRAY_PUSH(type, arr, val) ({ \
	((type*)arr->ptr)[arr->len] = val; \
	arr->len += 1; \
})
#define ARRAY_GET(type, arr, idx) (((type*)res->ptr)[idx])
#define ARRAY_FREE(type, arr, free_func) { \
	for (int i = 0; i < arr->len; i++) { \
		free_func(((type*)arr->ptr)[i]); \
	} \
	free(arr->ptr); \
	free(arr); \
}


#endif // FFISHIM_LIBRARY
