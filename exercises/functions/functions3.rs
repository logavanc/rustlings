// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me(9);
}

fn call_me(num: u128) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
