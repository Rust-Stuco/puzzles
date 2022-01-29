02_rust_basics
==============

Tasks
-----

The only file to edit is [`src/lib.rs`](/02_rust_basics/src/lib.rs).

We'll start by practicing with `struct`s and a simple `impl` block.

### Task 1

Revise the definition for the `Color` struct so that the test cases pass
and you'll be able to implement the stubbed out functions.

### Task 2

Implement `Color::new` so that it construct an instance of `Color` with
the given channel values.

### Task 3

Implement `Color::add` according to the following expression for additive
color:

sqrt((x^2 + y^2) / 2)

You should apply this separately to each channel and combine them into a
new `Color` instance.

To avoid overflow you will probably want to temporarily use a larger
integer type. For basic conversions between integer types you can use
`as`, e.g., `1u8 as u32`.

### Task 4

Now we'll practice working with slices (of `i32`). The first task here
is to implement a function to sum up all of the values in a given slice.

If you're stuck, taking a look at the slides is a good idea.

### Task 5

Oftentimes it is useful to have intermediate sums for various algorithms
(e.g., to answer queries about the sum over a given range). For this
task you'll implement a function which takes a slice `xs` and sets the
value at position `i` in `xs` to the sum of all preceding values in the
original `xs`, i.e.,

`xs'[i]` = `xs[0]` + `xs[1]` + ... + `xs[i]`

where `xs'` is the value of `xs` after the function returns.

### Task 6

Now for some practice with enums. Take a look at the given `Tree` enum.
You might notice that the left and right subtrees aren't simply a
`Tree`, but instead `Box<Tree>`---this is simply a type for safe, owned
pointers, so if you want to access the stored value you can just
dereference it (we need it because otherwise `Tree` would be infinitely
sized, similar to why we need a level of indirection for a linked list).

Implement `leftmost_leaf_iter`, which should return a reference to the
leftmost leaf in the tree, finding it iteratively.

### Task 7

Implement `leftmost_leaf_recur`, which should do the same, except
recursively.

### Task 8

Implement `max_and_min`, which should return the max and min elements
from the tree (in the format `(max, min)`). You might find
`std::cmp::min` and `max` useful.

### Task 9

Implement `sum`, which should return the sum of all the values in the
tree.

Grading
-------

To run: navigate to this directory and run `cargo test`. Your code
passes if it both compiles and passes all the provided test cases.

If you want to just run a certain test case you can do so with `cargo
test <test case name>`.

Once you have confirmed that your code builds and runs, turn in the
`lib.rs` file to Gradescope.

