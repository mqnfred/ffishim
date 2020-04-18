FFISHIM_SRC = $(shell find ffishim ffishim_derive -type f)
TESTS = $(patsubst %,%/success,$(shell find tests -mindepth 1 -maxdepth 1 -type d))

all: $(TESTS)

tests/%/success: tests/%/expected_output tests/%/actual_output
	diff $^
	touch $@

tests/%/actual_output: tests/%/a.out
	$< > $@

tests/%/a.out: tests/%/main.c target/debug/lib%.a
	gcc -Wall -g -Os $(word 1,$^) -o $@ \
		-Ltarget/debug \
		-l$(patsubst target/debug/lib%.a,%,$(word 2,$^))

target/debug/lib%.a: tests/%/src/lib.rs tests/%/Cargo.toml $(FFISHIM_SRC)
	cargo build --package $(patsubst target/debug/lib%.a,%,$@)

clean:
	# Anything that needs cleaning can be listed in the
	# .gitignore file. The `git clean -fdX` call removes all
	# files listed in the .gitignore.
	git clean -fdX
