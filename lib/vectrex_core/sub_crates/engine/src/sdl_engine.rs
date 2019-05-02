
use vectrex_os_shell::{ShellWindow, WindowProps};
use vectrex_logging::log::info;
use crate::traits::Engine;



const VERSION: &str = env!("CARGO_PKG_VERSION");



pub struct SdlEngine
{
    window: ShellWindow
}



impl SdlEngine
{
    pub fn new(props: WindowProps) -> Result<SdlEngine, String>
    {
        info!("Starting game engine, version: {}.", VERSION);

        let window = ShellWindow::new(props)?;
        Ok(SdlEngine { window })
    }
}



impl Engine for SdlEngine
{
    fn version(&self) -> String
    {
        VERSION.to_string()
    }

    fn run(&mut self)
    {
        info!("Running game event loop.");
        self.window.event_loop();
    }
}



impl Drop for SdlEngine
{
    fn drop(&mut self)
    {
        info!("Game engine shutting down.");
    }
}
