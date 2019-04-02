
use std::{ cmp::{ Ord, Ordering }, collections::BinaryHeap, os::raw::c_void };

use sdl2::{ VideoSubsystem, video::{ Window, GLContext, GLProfile::Core } };
use gl;

use crate::shader::Shader;
use crate::model::Model;



pub trait RenderObject
{
    #[inline]
    fn bind(&self);

    #[inline]
    fn unbind(&self);
}



#[derive(Eq)]
struct RenderCommand
{
    // Shader
    // Variables
    // Model
    // Transform matrix.
}



impl Ord for RenderCommand
{
    fn cmp(&self, _other: &RenderCommand) -> Ordering
    {
        Ordering::Equal
    }
}



impl PartialOrd for RenderCommand
{
    fn partial_cmp(&self, other: &RenderCommand) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}



impl PartialEq for RenderCommand
{
    fn eq(&self, _other: &RenderCommand) -> bool
    {
        true
    }
}



pub struct Graphics
{
    pub gl_context: GLContext,
    // render_objects: BinaryHeap<RenderCommand>
}



impl Graphics
{
    pub fn new(sdl_video: &VideoSubsystem, window: &Window) -> Graphics
    {
        let gl_context = window.gl_create_context().unwrap();

        gl::load_with(|s|
            {
                sdl_video.gl_get_proc_address(s) as *const c_void
            });

        let gl_attr = sdl_video.gl_attr();

        gl_attr.set_context_profile(Core);
        gl_attr.set_context_version(4, 2);

        unsafe
        {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        Graphics
        {
            gl_context,
            //render_objects: BinaryHeap::new()
        }
    }

    pub fn resize_view(&self, width: i32, height: i32)
    {
        unsafe
        {
            gl::Viewport(0, 0, width, height);
        }
    }

    pub fn render(&self)
    {
        unsafe
        {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
