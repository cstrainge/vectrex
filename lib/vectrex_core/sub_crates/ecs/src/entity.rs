
use crate::system::{ SystemId, System };



#[derive(Clone, PartialEq, Eq)]
pub struct EntityId(u64);



impl SystemId for EntityId
{
    fn new(generation: u32, index: u32) -> EntityId
    {
        EntityId(((generation as u64) << 32) | (index as u64))
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



pub struct Entity
{
}



impl Entity
{
}
