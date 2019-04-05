
use crate::system::SystemId;



#[derive(Clone, PartialEq, Eq)]
pub struct ComponentId(u64);



impl SystemId for ComponentId
{
    fn new(generation: u32, index: u32) -> ComponentId
    {
        ComponentId(((generation as u64) << 32) | (index as u64))
    }

    fn generation(&self) -> u32
    {
        ((self.0 & 0xFFFFFFFF00000000) >> 32) as u32
    }

    fn index(&self) -> usize
    {
        (self.0 & 0x00000000FFFFFFFF) as usize
    }
}



pub trait Component
{
    type ImplType;
    type SystemType;

    fn new(id: ComponentId, system: Box<Self::SystemType>) -> Self::ImplType;

    fn update(&mut self);

    fn mark_dead(&mut self);
    fn is_dead(&self) -> bool;
}
