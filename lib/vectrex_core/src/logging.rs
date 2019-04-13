
use std::{ fs::File, io::Write, sync::Mutex };
use log::{ Log, set_boxed_logger, set_max_level, Level, Metadata, Record };



struct Logger
{
    log_mutex: Mutex<File>
}



impl Logger
{
    fn new() -> Result<Logger, String>
    {
        let log_file = File::create("./vectrex-log.txt").map_err(|error| error.to_string())?;
        Ok(Logger { log_mutex: Mutex::new(log_file) })
    }
}



impl Log for Logger
{
    fn enabled(&self, _metadata: &Metadata) -> bool
    {
        true
    }

    fn log(&self, record: &Record)
    {
        let level = record.level().to_string();
        let module_path = record.module_path().unwrap_or_default();
        let args = record.args();

        match self.log_mutex.lock()
        {
            Ok(mut log_file) => write!(log_file, "{} | {} | {}", level, module_path, args).unwrap(),
            _                => println!("** LOG WRITE ERROR **")
        }

        println!("{} | {} | {}", level, module_path, args);

    }

    fn flush(&self)
    {
        let mut log_file = self.log_mutex.lock().unwrap();
        log_file.flush().unwrap();
    }
}



pub fn init_logging() -> Result<(), String>
{
    let logger = Logger::new()?;

    match set_boxed_logger(Box::new(logger))
    {
        Err(error) =>
            {
                let message = format!("Could not register engine logger: {}", error.to_string());
                return Err(message);
            },

        _ => {}
    }

    set_max_level(Level::Trace.to_level_filter());

    Ok(())
}
