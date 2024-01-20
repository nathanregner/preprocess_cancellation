use std::env;

fn main() {
    preprocess_cancellation::main(env::args_os()).unwrap();
}
