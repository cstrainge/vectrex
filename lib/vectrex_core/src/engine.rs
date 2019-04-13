
use crate::{ logging::init_logging, log::info, os::shell::{ WindowProps, ShellWindow }};



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
        info!("Engine log system started.");

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
