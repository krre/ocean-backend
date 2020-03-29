use super::Controller;

#[derive(Default)]
pub struct Topic {
    name: String,
    description: String,
}

impl Topic {}

impl Controller for Topic {
    fn new() -> Topic {
        Default::default()
    }

    fn exec(&self) {
        println!("exec");
    }
}
