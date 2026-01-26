pub mod classic;
pub mod radial;
pub mod spatial;

pub trait Shell {
    fn run(&self);
}
