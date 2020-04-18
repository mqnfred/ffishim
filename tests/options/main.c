#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "../../ffishim/header.h"
#include "../helpers.h"

extern outcome_t *ffi_give_option();
extern outcome_t *ffi_take_option(int32_t *opt);
extern outcome_t *ffi_string_option(char **s);
extern outcome_t *ffi_option_result(int32_t *opt);

int main() {
	outcome_t *outcome_1 = ffi_give_option();
	PRINTO("ffi_give_option() = ", "%p", int32_t*, outcome_1);
	printf("*ffi_give_option() = %d\n", **(int32_t**)outcome_1->payload);
	free(*(void**)outcome_1->payload);
	free_outcome(outcome_1);

	SEPARATOR;

	int *opt = malloc(sizeof(int32_t));
	*opt = 5;
	outcome_t *outcome_2 = ffi_take_option(opt);
	printf("ffi_take_option(&5) = %p\n", *(int32_t**)outcome_2->payload);
	free_outcome(outcome_2);

	outcome_t *outcome_3 = ffi_take_option(NULL);
	printf("ffi_take_option(NULL) = %d\n", **(int32_t**)outcome_3->payload);
	free(*(void**)outcome_3->payload);
	free_outcome(outcome_3);

	SEPARATOR;

	char **str = malloc(sizeof(char*));
	(*str) = malloc(sizeof(char)*4);
	(*str)[0] = 'f';
	(*str)[1] = 'f';
	(*str)[2] = 'i';
	(*str)[3] = '\0';
	outcome_t *outcome_4 = ffi_string_option(str);
	printf(
		"*ffi_string_option(&\"ffi\") = %s\n",
		**(char***)outcome_4->payload
	);
	free(**(void***)outcome_4->payload);
	free(*(void**)outcome_4->payload);
	free_outcome(outcome_4);

	outcome_t *outcome_5 = ffi_string_option(NULL);
	printf("ffi_string_option(NULL) = %p\n", *(char***)outcome_5->payload);
	free_outcome(outcome_5);

	SEPARATOR;

	int *opt6 = malloc(sizeof(int32_t));
	*opt6 = 5;
	outcome_t *outcome_6 = ffi_option_result(opt6);
	printf("*ffi_option_result(&5) = %d\n", *(int32_t**)outcome_6->payload);
	free_outcome(outcome_6);

	outcome_t *outcome_7 = ffi_option_result(NULL);
	printf("ffi_option_result(NULL) = %s\n", outcome_7->message);
	free_outcome(outcome_7);

	return 0;
}
