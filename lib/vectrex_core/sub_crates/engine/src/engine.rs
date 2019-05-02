
use vectrex_os_shell::{ WindowProps };
use vectrex_logging::init_logging;
use crate::traits::Engine;
use crate::sdl_engine::SdlEngine;




pub fn new_engine(props: WindowProps) -> Result<Box<dyn Engine>, String>
{
    init_logging()?;

    let engine = SdlEngine::new(props)?;
    Ok(Box::new(engine))
}
