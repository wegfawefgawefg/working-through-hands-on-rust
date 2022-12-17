#![warn(clippy::all, clippy::pedantic)]

// demonstrate function chaining
fn do_thing(x: i32) -> i32 {
    x + 1
}

fn main() {
    let a = 1;
    let a = do_thing(a);
    let a = do_thing(a);
    let a = do_thing(a);

    println!("{}", a);
}
