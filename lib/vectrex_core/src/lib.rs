
extern crate vectrex_logging;
extern crate vectrex_engine;
extern crate vectrex_os_shell;
extern crate vectrex_ecs;
extern crate vectrex_gfx;
extern crate vectrex_gfx_gui;
extern crate vectrex_srl;



pub mod logging
{
    pub use vectrex_logging::*;
}



pub mod engine
{
    pub use vectrex_engine::engine::*;
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



pub mod ecs
{
    pub use vectrex_ecs::*;
}



pub mod srl
{
    pub use vectrex_srl::*;
}
