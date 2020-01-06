#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;

mod common;
mod attack;
mod position;
mod util;
mod gen;
mod game;
mod search;
mod eval;

/*

export RUST_BACKTRACE=1
cargo run
*/

fn main() {
    game::game();
    //python::main();
}
