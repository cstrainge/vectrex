
use std::{ cell::RefCell, fs::File, io::{ Write, BufWriter }, sync::Mutex };
use log::{ Log, set_boxed_logger, Metadata, Record };



struct Logger
{
    log_writer: Mutex<RefCell<BufWriter<File>>>
}



impl Logger
{
    fn new() -> Result<Logger, String>
    {
        let log_file = File::create("./vectrex-log.txt").map_err(|error| error.to_string())?;
        let log_writer = BufWriter::new(log_file);

        let logger = Logger
            {
                log_writer: Mutex::new(RefCell::new(log_writer))
            };

        Ok(logger)
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
        let writer_mutex = self.log_writer.lock().unwrap();
        let mut writer_ref = writer_mutex.borrow_mut();

        writer_ref.write_fmt(format_args!("{} | {} | {}",
                                      record.target(),
                                      record.level(),
                                      record.args())).unwrap();
    }

    fn flush(&self)
    {
        let writer_mutex = self.log_writer.lock().unwrap();
        let mut writer_ref = writer_mutex.borrow_mut();

        writer_ref.flush().unwrap();
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

    Ok(())
}
