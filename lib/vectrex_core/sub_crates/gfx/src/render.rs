
use std::{ cmp::{ Ord, Ordering }, collections::BinaryHeap, os::raw::c_void };

use sdl2::{ VideoSubsystem, video::{ Window, GLContext, GLProfile::Core } };
use gl;

use crate::shader::Shader;
use crate::model::Model;



pub trait Bindable
{
    #[inline]
    fn bind(&self);

    #[inline]
    fn unbind(&self);
}


#[derive(Eq)]
struct RenderCommand
{
    shader: Option<Shader>,
    // Variables
    model: Option<Model>,
    transform: Option<f32>
}



fn some_matcher<type_t, result_t> (a: Option<type_t>, b: Option<type_t>, default: result_t
                                   checker: &Fn(type_t, type_t)-> result_t) -> result_t
    where type_t: PartialEq
{
    match (a, b)
        {
            (Some(a_some), Some(b_some)) =>
                {
                    checker(a_some, b_some)
                }

            (_, _) => default
        }
}



impl PartialEq for RenderCommand
{
    fn eq(&self, other: &RenderCommand) -> bool
    {
        let result = match (self.shader, other.shader)
            {
                (Some(this_shader), Some(other_shader)) =>
                    {
                        this_shader == other_shader
                    }

                    (_, _) => false
            }

            &&

            match (self.model, other.model)
                {
                    (Some(this_model), Some(other_model)) =>
                        {
                            this_model == other_model
                        }

                    (_, _) => false
                }

            &&

            match (self.transform, other.transform)
                {
                    (Some(this_transform), Some(other_transform)) =>
                        {
                            this_transform == other_transform
                        }

                    (_, _) => false
                };

        result
    }
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
