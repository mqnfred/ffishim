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
#include <stdio.h>

int main() {
	printf("test passes: %d\\n", 1);
	return 0;
}
EOF

cat > ${dir}/expected_output <<EOF
test passes: 1
EOF
