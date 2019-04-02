
use std::mem::size_of;
use gl::{ self, types::{ GLuint, GLsizeiptr, GLvoid } };

use crate::render::RenderObject;



pub struct Model
{
    vertex_buffer_object: GLuint
}



impl Model
{
    pub fn new() -> Model
    {
        Model
        {
            vertex_buffer_object: 0
        }
    }

    pub fn new_from_data(vertices: &Vec<f32>) -> Model
    {
        let mut vertex_buffer_object: GLuint = 0;

        let vertex_size = (vertices.len() * size_of::<f32>()) as GLsizeiptr;
        let vertices_ptr = vertices.as_ptr() as *const GLvoid;

        unsafe
        {
            gl::GenBuffers(1, &mut vertex_buffer_object);

            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
            gl::BufferData(gl::ARRAY_BUFFER, vertex_size, vertices_ptr, gl::STATIC_DRAW);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Model
        {
            vertex_buffer_object
        }
    }
}



impl RenderObject for Model
{
    #[inline]
    fn bind(&self)
    {
        unsafe
        {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_object);
        }
    }

    #[inline]
    fn unbind(&self)
    {
        unsafe
        {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}
