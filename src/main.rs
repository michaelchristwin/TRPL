mod control_flow;
mod ownership;
use crate::control_flow::{cf, lp};
use crate::ownership::ownership;

fn main() {
    cf();
    lp();
    ownership();
}
