

use gl;

use crate::render::Bindable;



#[derive(PartialEq, Eq)]
pub struct Material
{
}



impl Material
{
    pub fn new() -> Material
    {
        Material
        {
        }
    }
}



impl Bindable for Material
{
    fn bind(&self)
    {
    }

    fn unbind(&self)
    {
    }
}
