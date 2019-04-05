
use crate::component::Component;
use crate::entity::{ EntityId, Entity };



pub trait SystemId
{
    fn new(generation: u32, index: u32) -> Self;

    fn generation(&self) -> u32;
    fn index(&self) -> usize;
}



struct CollectionItem<DataType>
{
    pub generation: u32,
    pub data: DataType
}



pub struct SystemCollection<IndexType: SystemId, DataType>
{
    items: Vec<CollectionItem<DataType>>
}



impl<IndexType: SystemId, DataType> SystemCollection<IndexType, DataType>
{
    fn new() -> SystemCollection<IndexType, DataType>
    {
        SystemCollection::<IndexType, DataType> { items: Vec::new() }
    }

    fn get(&self, id: &IndexType) -> Option<&mut DataType>
    {
        let found = &mut self.items[id.index()];

        if id.generation() != found.generation
        {
            return None;
        }

        Some(&mut found.data)
    }
}



pub struct System
{
    //pub components: SystemCollection<Component>,
    pub entities: SystemCollection<EntityId, Entity>
}



impl System
{
}
