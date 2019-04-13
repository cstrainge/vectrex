
extern crate vectrex_os_shell;
extern crate vectrex_ecs;
extern crate vectrex_gfx;
extern crate vectrex_gfx_gui;
extern crate vectrex_srl;

pub extern crate log;



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



mod logging;
pub mod engine;
