FFISHIM_SRC = $(shell find ffishim ffishim_derive -type f)
TESTS = $(patsubst %,%/success,$(shell find tests -mindepth 1 -maxdepth 1 -type d))

all: $(TESTS)
	@echo "-----------------------------"
	@echo "--- All tests successful! ---"
	@echo "-----------------------------"

tests/%/success: tests/%/expected_output tests/%/actual_output
	diff $^
	touch $@

tests/%/actual_output: tests/%/a.out
	valgrind --error-exitcode=1 --leak-check=full $< | tee $@

tests/%/a.out: tests/%/main.c target/debug/lib%.a tests/helpers.h
	gcc -Wall -g -Os -o $@ $(word 1,$^) $(word 2,$^) -ldl -lm -pthread

target/debug/lib%.a: tests/%/src/lib.rs tests/%/Cargo.toml $(FFISHIM_SRC)
	cargo build --package $(patsubst target/debug/lib%.a,%,$@)

clean:
	# Anything that needs cleaning can be listed in the
	# .gitignore file. The `git clean -fdX` call removes all
	# files listed in the .gitignore.
	git clean -fdX
