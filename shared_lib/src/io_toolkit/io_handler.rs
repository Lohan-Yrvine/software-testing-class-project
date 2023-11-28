use std::fs;
use std::io;
use std::path::Path;

use anyhow::Result;

pub struct IOHandler<R, W> {
    reader: R,
    writer: W,
}

pub trait DefaultIOHandler {
    fn default() -> IOHandler<io::StdinLock<'static>, io::Stdout>;
}

impl<R, W> IOHandler<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }

    pub fn read_line(&mut self) -> Result<String> {
        let mut result = String::new();
        self.reader.read_line(&mut result)?;
        Ok(result)
    }

    pub fn write(&mut self, msg: &str) -> Result<()> {
        write!(self.writer, "{}", msg)?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn set_remove_file_on_exit_handler(&self, path: String) -> Result<()> {
        ctrlc::set_handler(move || {
            println!("\n\nEncerrando programa...");
            if Path::new(&path).exists() {
                fs::remove_file(&path).unwrap();
            }
            println!("Programa encerrado.");
            std::process::exit(0);
        })?;

        Ok(())
    }
}

impl DefaultIOHandler for IOHandler<io::StdinLock<'static>, io::Stdout> {
    fn default() -> IOHandler<io::StdinLock<'static>, io::Stdout> {
        Self {
            reader: io::stdin().lock(),
            writer: io::stdout(),
        }
    }
}
