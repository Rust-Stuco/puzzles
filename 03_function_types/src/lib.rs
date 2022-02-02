#![allow(dead_code)]

fn task_1_function() {}

fn make_task_2_closure() -> impl FnMut() -> i32 {
    todo!("Task 2")
}

#[derive(Debug, PartialEq, Eq)]
enum Names {
    Foo,
    Bar,
    Baz,
    Qux,
}

fn task_3_function() {}

// Uncomment the following code if attempting Task 4, only edit the specified section
/*
/* BEGIN EDIT SECTION */
fn task_4_function(f: impl Fn(&String, &String) -> &String) -> String {
/* END EDIT SECTION */
    let x = String::from("x");
    let y = String::from("y");
    f(&x, &y).clone()
}
*/
// Comment this function out if actually attempting Task 4; this is just here to make the code
// compile when TASK_4_ATTEMPTED is set to false
fn task_4_function<T>(f: &T) -> String {
    String::from("foobar")
}

/// Change this to `true` if you'd like to attempt the extra credit
const TASK_4_ATTEMPTED: bool = false;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Task 1: make a function that can be passed in as a function pointer
    /// with the signature `fn(i32) -> i32`, returning 1337 when 42 is passed
    fn task_1() {
        fn takes_function_pointer(f: fn(i32) -> i32) {
            assert_eq!(f(42), 1337);
        }

        takes_function_pointer(task_1_function);
    }

    #[test]
    /// Task 2: Make a closure that changes state when called! Specifically,
    /// the closure should return the value 2^{x-1}, where x is the number of
    /// times the function has been called.
    fn task_2() {
        fn takes_closure(mut f: impl FnMut() -> i32) {
            assert_eq!(f(), 1);
            assert_eq!(f(), 2);
            assert_eq!(f(), 4);
        }

        takes_closure(make_task_2_closure())
    }

    #[test]
    /// Task 3: Make a function that, using a mutable reference, cycles through
    /// variants of an enum
    fn task_3() {
        let mut x = Names::Foo;
        task_3_function(&mut x);
        assert_eq!(x, Names::Bar);
        task_3_function(&mut x);
        assert_eq!(x, Names::Baz);
        task_3_function(&mut x);
        assert_eq!(x, Names::Qux);
        task_3_function(&mut x);
        assert_eq!(x, Names::Foo);
    }

    #[test]
    /// Task 4: Extra credit, passes automatically unless `TASK_4_ATTEMPTED` is set to true
    /// This task is about using lifetimes in trait specifiers for function traits. Very tricky,
    /// but the compiler error message has gotten a lot better
    fn task_4() {
        fn get_first<'a>(x: &'a String, _y: &'a String) -> &'a String {
            x
        }
        fn get_second<'a>(_x: &'a String, y: &'a String) -> &'a String {
            y
        }

        if (TASK_4_ATTEMPTED) {
            assert_eq!(task_4_function(get_first), "x");
            assert_eq!(task_4_function(get_second), "y");
        }
    }
}
