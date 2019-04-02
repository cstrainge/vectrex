
use sdl2::{ VideoSubsystem, video::{ Window, GLContext } };
use gl;



pub trait RenderObject
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
    pub fn new(sdl_video: &VideoSubsystem, window: &Window) -> Graphics
    {
        let gl_context = window.gl_create_context().unwrap();

        gl::load_with(|s|
            {
                sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void
            });

        let gl_attr = sdl_video.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 2);

        unsafe
        {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        Graphics
        {
            gl_context
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
