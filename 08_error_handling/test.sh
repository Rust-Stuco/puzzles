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
if (( $(echo -ne "69\n1337\n" | cargo run) != 7257550166497394936 )) ; then exit 1 ; fi
if (( $(echo -ne "420\n15251\n" | cargo run) != 11604359411781646366 )) ; then exit 1 ; fi
if (( $(echo -ne "98008\n98013\n" | cargo run) != 12880269392473839412 )) ; then exit 1 ; fi
if (( $(echo -ne "9223372036854775908\n18446744073709427184\n" | cargo run) != 9144881374108674624 )) ; then exit 1 ; fi
