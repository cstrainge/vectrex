

pub trait SystemId
{
    type IdType;

    fn new(generation: u32, index: u32) -> Self::IdType;
    fn generation(&self) -> u32;
    fn index(&self) -> u32;
}



pub struct System
{
}



impl System
{
}
