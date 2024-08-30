mod control_flow;
mod ownership;
use crate::control_flow::{cf, lp};
use crate::ownership::{first_word, ownership};

fn main() {
    cf();
    lp();
    ownership();
    println!("{}", first_word("hello world"));
}
