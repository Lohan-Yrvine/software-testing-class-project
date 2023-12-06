use std::fmt::Display;
use std::io;

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

    pub fn write<T: Display>(&mut self, msg: T) -> Result<()> {
        write!(self.writer, "{}", msg)?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn writer(&self) -> &W {
        &self.writer
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
