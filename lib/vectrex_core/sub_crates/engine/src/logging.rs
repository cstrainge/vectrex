
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

        let message = format!("{} | {} | {}", level, module_path, args);
        let message_bytes = message.as_bytes();

        {
            let mut log_file = self.log_mutex.lock()
                .unwrap_or_else(|err| panic!("** LOG ACCESS ERROR: {} **", err.to_string()));

            let written = log_file.write(message.as_bytes())
                .unwrap_or_else(|err| panic!("** LOG WRITE ERROR: {} **", err.to_string()));

            if written != message_bytes.len()
            {
                panic!("** LOG WRITE TRUNCATED: ({}/{}). **", written, message_bytes.len());
            }
        }

        println!("{}", message);
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
