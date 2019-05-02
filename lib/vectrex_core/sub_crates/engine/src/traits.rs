
pub trait Engine
{
    fn version(&self) -> String;
    fn run(&mut self);
}
