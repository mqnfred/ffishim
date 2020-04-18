#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "../../ffishim/header.h"
#include "../helpers.h"

extern outcome_t *ffi_give_string();
extern outcome_t *ffi_take_string(const char *s);

int main() {
	outcome_t *outcome_1 = ffi_give_string();
	PRINTO("ffi_give_string() = ", "%s", const char*, outcome_1);
	free(*(void**)(outcome_1->payload));
	free_outcome(outcome_1);

	SEPARATOR;

	char *str = malloc(sizeof(char)*4);
	str[0] = 'f';
	str[1] = 'f';
	str[2] = 'i';
	str[3] = '\0';
	outcome_t *outcome_2 = ffi_take_string(str);
	PRINTO("ffi_take_string(\"ffi\") = ", "%s", const char*, outcome_2);
	free(*(void**)(outcome_2->payload));
	free_outcome(outcome_2);

	return 0;
}
