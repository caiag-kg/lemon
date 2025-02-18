#![allow(unused_variables)]
mod args;
use args::get_matches;


fn main() {
    let matches = get_matches().unwrap();
    println!("{:?}", matches.value_source("t"));
}
