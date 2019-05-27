
#![allow(dead_code)]



pub mod eventing;
pub mod graphics;



pub mod interface
{
    use crate::eventing::{ WindowHandler, MouseHandler, KeyboardHandler };
    use crate::graphics::RenderHandler;



    pub trait InterfaceLayer: WindowHandler + MouseHandler + KeyboardHandler + RenderHandler
    {
        fn name(&self) -> String;

        fn is_visible(&self) -> bool;
        fn is_active(&self) -> bool;

        fn is_capturing_mouse(&self) -> bool;
        fn is_capturing_keyboard(&self) -> bool;

        fn is_event_exclusive(&self) -> bool;
        fn is_draw_exclusive(&self) -> bool;
    }
}
