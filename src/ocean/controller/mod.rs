pub mod topic;

pub trait Controller {
    fn new() -> Self;
    fn exec(&self);
}
