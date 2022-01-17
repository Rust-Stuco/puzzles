/* BEGIN EDIT SECTION */
use std::io;

/// Reads a single line from standard input. Rewrite this so that it cannot
/// panic due to malformed input!
fn goal1() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer);
    
    buffer
}

/// Parses two strings as two `u64`s. Rewrite this so it can't panic due to
/// malformed input!
fn goal2(s1: &str, s2: &str) -> (u64, u64) {
    let i1 = s1.trim().parse();
    let i2 = s2.trim().parse();

    (i1, i2)
}

/// Hashes two integers using a modification of Xorshift. Rewrite so that all
/// necessary operations are wrapping!
fn goal3(i1: u64, i2: u64) -> u64 {
    let mut x = i1 + i2;

    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;

    x * 0x9e3779b97f4a7c16
}

fn main() {
    let s1 = goal1();
    let s2 = goal1();

    let (i1, i2) = goal2(&s1, &s2);

    println!("{}", goal3(i1, i2));
}
/* END EDIT SECTION */
