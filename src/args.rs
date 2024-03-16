#[derive(Debug)]
pub struct Args {
    pub bilde_1: String,
    pub bilde_2: String,
    pub output: String,
}

impl Args {
    pub fn new() -> Self {
        Args {
            bilde_1: få_nth_arg(1),
            bilde_2: få_nth_arg(2),
            output: få_nth_arg(3),
        }
    }
}

fn få_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}
