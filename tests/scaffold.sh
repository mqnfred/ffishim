#!/bin/sh

if [ "${1}" = "-h" ] || [ "${1}" = "--help" ] || [ -z "${1}" ]; then
	echo "Please provide a test name: ${0} <desired_test_name>"
	echo "This will create a test in tests/<desired_test_name>"
	exit 127
else
	name="${1}"
	dir="tests/${name}"
fi

sed -i "s#^]\$#\t\"${dir}\",\n]#g" Cargo.toml
cargo new --lib ${dir}

cat >> ${dir}/Cargo.toml <<EOF
ffishim = { path = "../../ffishim" }
ffishim_derive = { path = "../../ffishim_derive" }

[lib]
name = "${name}"
crate-type = ["staticlib"]
EOF

cat > ${dir}/main.c <<EOF
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "../../ffishim/src/library/header.h"
#include "../helpers.h"

int main() {
	PRINTO("ffi_add(1, 3) = ", "%ld", int64_t, ffi_add(1, 3));
	SEPARATOR;

	return 0;
}
EOF

cat > ${dir}/src/lib.rs <<EOF
#[macro_use]
extern crate ffishim_derive;

#[ffishim]
fn add(a: i64, b: i64) -> i64 {
    a + b
}
EOF

cat > ${dir}/expected_output <<EOF
ffi_add(1, 3) = 4
---
EOF
