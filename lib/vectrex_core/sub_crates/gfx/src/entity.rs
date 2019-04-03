
use gl;

use crate::render::Bindable;
use crate::shader::Shader;
use crate::model::Model;



pub struct GfxEntity
{
    pub position: [ f32; 3 ],
    pub rotation: [ f32; 3 ],
    pub scale: f32,

    model: Model,
    shader: Shader
}



impl GfxEntity
{
    pub fn new() -> GfxEntity
    {
        GfxEntity
        {
            position: [ 0.0, 0.0, 0.0 ],
            rotation: [ 0.0, 0.0, 0.0 ],
            scale: 1.0,

            model: Model::new(),
            shader: Shader::new()
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32)
    {
        self.position[0] = x;
        self.position[1] = y;
        self.position[2] = z;
    }

    pub fn move_relative(&mut self, x: f32, y: f32, z: f32)
    {
        self.position[0] += x;
        self.position[1] += y;
        self.position[2] += z;
    }

    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32)
    {
        self.rotation[0] = x;
        self.rotation[1] = y;
        self.rotation[2] = z;
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32)
    {
        self.rotation[0] += x;
        self.rotation[1] += y;
        self.rotation[2] += z;
    }
}



impl Bindable for GfxEntity
{
    fn bind(&self)
    {
        self.shader.bind();
        self.model.bind();
    }

    fn unbind(&self)
    {
    }
}
