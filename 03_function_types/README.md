# 03_function_types

## Tasks
The only file to edit is [`src/lib.rs`](/03_function_types/src/lib.rs).

### Task 1

Make a function that can be passed in as a function pointer with the signature `fn(i32) -> i32`, returning the value 1337 when 42 is passed.

### Task 2

Make a closure that changes state when called! Specifically, the closure should return the value 2^{x-1}, where x is the number of times the function has been called.

### Task 3

Make a function that, using a mutable reference, cycles through variants of an enum

### Task 4 (EXTRA CREDIT)

Use higher-order trait specifiers to take in a function that has lifetime parameters.

Task 4 will pass even if this is not attempted, so long as the `TASK_4_ATTEMPTED` boolean is set to `false`.

## Grading
To run: navigate to this directory and run `cargo test`. Your code passes if it both compiles and does not crash while running.

Once you have confirmed that your code builds and passes all the tests, turn in just the `lib.rs` file to Gradescope.
