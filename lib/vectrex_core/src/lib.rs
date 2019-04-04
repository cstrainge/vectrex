
extern crate vectrex_os_shell;
extern crate vectrex_ecs;
extern crate vectrex_gfx;
extern crate vectrex_gfx_gui;
extern crate vectrex_srl;



pub mod ecs
{
    pub use vectrex_ecs::*;
}



pub mod os
{
    pub mod shell
    {
        pub use vectrex_os_shell::*;
    }
}



pub mod gfx
{
    pub use vectrex_gfx::*;

    pub mod gui
    {
        pub use vectrex_gfx_gui::*;
    }
}



pub mod srl
{
    pub use vectrex_srl::*;
}



pub mod engine
{
    use crate::os::shell::{ WindowProps, ShellWindow };



    const VERSION: &str = env!("CARGO_PKG_VERSION");



    pub struct Engine
    {
        window: ShellWindow
    }



    impl Engine
    {
        pub fn new(props: WindowProps) -> Engine
        {
            Engine
            {
                window: ShellWindow::new(props)
            }
        }

        pub fn version(&self) -> &'static str
        {
            VERSION
        }

        pub fn run(&mut self)
        {
            self.window.event_loop();
        }
    }
}
