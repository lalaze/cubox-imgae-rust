use std::env;
mod cubox;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    cubox::get_box(args[1].to_string(), args[2].to_string())
}
