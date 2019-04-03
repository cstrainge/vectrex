

use gl::{ self, types::{ GLuint } };

use crate::render::Bindable;



#[derive(PartialEq, Eq)]
pub struct Shader
{
    program_id: GLuint
}



impl Shader
{
    pub fn new() -> Shader
    {
        Shader
        {
            program_id: 0
        }
    }
}



impl Bindable for Shader
{
    fn bind(&self)
    {
    }

    fn unbind(&self)
    {
    }
}
