
use std::{ cmp::{ Ord, Ordering }, collections::BinaryHeap, os::raw::c_void };

use sdl2::{ VideoSubsystem, video::{ Window, GLContext, GLProfile::Core } };
use gl;



pub trait Bindable
{
    #[inline]
    fn bind(&self);

    #[inline]
    fn unbind(&self);
}



pub struct Graphics
{
    pub gl_context: GLContext
}



impl Graphics
{
    pub fn new(sdl_video: &VideoSubsystem, window: &Window) -> Result<Graphics, String>
    {
        let gl_context = window.gl_create_context()?;

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

        Ok(Graphics
            {
                gl_context
                //render_objects: BinaryHeap::new()
            })
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
