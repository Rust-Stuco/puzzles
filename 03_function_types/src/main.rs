/* Only edit between the sections marked:
/* BEGIN EDIT SECTION */
 * and
/* END EDIT SECTION */
 */

/// Goal 1: make a function that can be passed into this one!
fn takes_function_pointer(f: fn(i32) -> i32) {
    assert_eq!(f(42), 1337);
}

/* BEGIN EDIT SECTION */
fn goal1() {}
/* END EDIT SECTION */

/// Goal 2: make a closure that changes state when called!
fn takes_closure(mut f: impl FnMut() -> i32) {
    assert_eq!(f(), 1);
    assert_eq!(f(), 2);
    assert_eq!(f(), 4);
}

#[derive(Debug, PartialEq, Eq)]
enum Names {
    Foo,
    Bar,
    Baz,
    Qux,
}

/// Goal 3: Change the following function to be compatible with how it's used in `main`
/* BEGIN EDIT SECTION */
fn goal3() {}
/* END EDIT SECTION */

/// Goal 4 (Extra Credit): rewrite this function to use references instead!
/* BEGIN EDIT SECTION */
/*
fn takes_choose_with_lifetimes(f: impl Fn(&String, &String) -> &String)
{
    /* END EDIT SECTION */
    let x = String::from("x");
    {
        let y = String::from("y");

        // Need to convert to pointers to compare references without dereferencing
        assert_eq!(f(&x, &y).as_ptr(), x.as_ptr());
        assert_eq!(f(&y, &x).as_ptr(), y.as_ptr());
    }
}
*/

/* BEGIN EDIT SECTION */
#[allow(dead_code)]
fn goal4() {}
/* END EDIT SECTION */

fn main() {
    /* BEGIN EDIT SECTION */
    let goal2 = || {};
    /* EDIT EDIT SECTION */

    takes_function_pointer(goal1);
    takes_closure(goal2);

    let mut x = Names::Foo;
    goal3(&mut x);
    assert_eq!(x, Names::Bar);
    goal3(&mut x);
    assert_eq!(x, Names::Baz);
    goal3(&mut x);
    assert_eq!(x, Names::Qux);
    goal3(&mut x);
    assert_eq!(x, Names::Foo);

    /* BEGIN EDIT SECTION */
    // takes_choose_with_lifetimes(goal4);
    /* END EDIT SECTION */
}
