#!/bin/bash

set -euxo pipefail

# Test malformed UTF-8
echo -ne "\xba\xdc\n" | cargo run

# Test malformed first input
echo -ne "hello\n10\n" | cargo run

# Test malformed second input
echo -ne "10\nhello\n" | cargo run

# Test missing second input
echo -ne "foobar\n" | cargo run

# Test a few cases
[[ $(echo -ne "69\n1337\n" | cargo run) -eq 7257550166497394936 ]]
[[ $(echo -ne "420\n15251\n" | cargo run) -eq 11604359411781646366 ]]
[[ $(echo -ne "98008\n98013\n" | cargo run) -eq 12880269392473839412 ]]
[[ $(echo -ne "9223372036854775908\n18446744073709427184\n" | cargo run) -eq 9144881374108674624 ]]
