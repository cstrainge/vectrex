
extern crate vectrex_os_shell;
extern crate vectrex_ecs;
extern crate vectrex_gfx;
extern crate vectrex_gfx_gui;
extern crate vectrex_srl;

extern crate log;


mod logging;



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
    pub mod log
    {
        pub use crate::log::{ info, warn, error };
    }


    use crate::os::shell::{ WindowProps, ShellWindow };
    use crate::logging::init_logging;



    const VERSION: &str = env!("CARGO_PKG_VERSION");



    pub struct Engine
    {
        window: ShellWindow
    }



    impl Engine
    {
        pub fn new(props: WindowProps) -> Result<Engine, String>
        {
            init_logging()?;

            let window = ShellWindow::new(props)?;
            Ok(Engine { window: window })
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
