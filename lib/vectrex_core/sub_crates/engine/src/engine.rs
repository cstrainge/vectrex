
use vectrex_os_shell::{ WindowProps, ShellWindow };
use vectrex_logging::{ init_logging, log::info };




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
        info!("Starting game engine, version: {}.", VERSION);

        let window = ShellWindow::new(props)?;

        Ok(Engine { window: window })
    }

    pub fn version(&self) -> &'static str
    {
        VERSION
    }

    pub fn run(&mut self)
    {
        info!("Running game event loop.");
        self.window.event_loop();
    }
}



impl Drop for Engine
{
    fn drop(&mut self)
    {
        info!("Game engine shutting down.");
    }
}
