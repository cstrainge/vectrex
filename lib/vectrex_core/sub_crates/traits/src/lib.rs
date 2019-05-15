
#![allow(dead_code)]



pub mod eventing;
pub mod graphics;



pub mod interface
{
    use crate::eventing::{ WindowHandler, MouseHandler, KeyboardHandler };
    use crate::graphics::Renderable;



    pub trait InterfaceLayer: WindowHandler + MouseHandler + KeyboardHandler + Renderable
    {
        fn name(&self) -> String;

        fn is_visible(&self) -> bool;
        fn is_active(&self) -> bool;

        fn is_capturing_mouse(&self) -> bool;
        fn is_capturing_keyboard(&self) -> bool;

        fn is_exclusive(&self) -> bool;
    }
}
