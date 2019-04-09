
extern crate vectrex_core;
extern crate vectrex_worlds;

use vectrex_core::{ engine::Engine, os::shell::WindowProps };



fn main()
{
    let mut engine = Engine::new(WindowProps
            {
                title: "New Game +",
                full_screen: false,
                resizable: true,
                width: 800,
                height: 600
            })
            .unwrap_or_else(|error_message| panic!("Error starting engine: {}", error_message));

    engine.run();
}
