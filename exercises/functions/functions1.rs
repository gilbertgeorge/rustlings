// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.


fn main() {
    call_me();

    let x = plus_one(5);
}

fn call_me() {
    println!("call_me")
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}

