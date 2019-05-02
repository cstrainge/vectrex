
extern crate vectrex_core;

use vectrex_core::{ engine::new_engine, os::shell::WindowProps };



fn main()
{
    let mut engine = new_engine(WindowProps
        {
            title: "Vectrex Editor",
            full_screen: false,
            resizable: true,
            width: 800,
            height: 600
        })
        .unwrap_or_else(|error_message| panic!("Error starting engine: {}", error_message));

    engine.run();
}
