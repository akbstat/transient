use std::{
    cell::Cell,
    fs::OpenOptions,
    io::{BufReader, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Logger {
    filepath: PathBuf,
    cursor: Cell<u64>,
    stop: Cell<bool>,
}

impl Logger {
    pub fn new(filepath: &Path) -> anyhow::Result<Self> {
        if !filepath.exists() {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(filepath)?;
        }
        Ok(Logger {
            filepath: filepath.into(),
            cursor: Cell::new(0),
            stop: Cell::new(false),
        })
    }
    pub fn write(&self, content: &str) -> anyhow::Result<()> {
        let mut writer = OpenOptions::new().append(true).open(&self.filepath)?;
        let content = format!(
            "[{}][INFO] {}\n",
            chrono::Local::now().to_string().get(0..26).unwrap(),
            content
        );
        writer.write(content.as_bytes())?;
        Ok(())
    }

    pub fn read(&self) -> anyhow::Result<(String, bool)> {
        if self.stop.get() {
            return Ok((self.reading()?, false));
        }
        Ok((self.reading()?, true))
    }

    fn reading(&self) -> anyhow::Result<String> {
        let current_cursor = self.cursor.get();
        let mut reader = OpenOptions::new().read(true).open(&self.filepath)?;
        reader.seek(SeekFrom::Start(current_cursor))?;
        let mut buffer = BufReader::new(reader);
        let mut result = String::new();
        let cursor = buffer.read_to_string(&mut result)? as u64;
        self.cursor.set(current_cursor + cursor);
        Ok(result)
    }

    pub fn stop_logging(&self) {
        self.stop.set(true);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        thread,
        time::Duration,
    };

    use chrono::Local;

    use super::*;
    #[test]
    fn logger_test() -> anyhow::Result<()> {
        let now = Local::now();
        let filename = format!("{:?}", now);
        let filename = filename
            .get(0..19)
            .unwrap()
            .replace("-", "")
            .replace(":", "");
        let filepath =
            Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\logs").join(filename);
        let logger = Arc::new(Mutex::new(Logger::new(&filepath)?));

        let mut handlers = vec![];
        let stop = Arc::new(Mutex::new(false));
        let reader_logger = Arc::clone(&logger);
        let writer_logger = Arc::clone(&logger);
        let reader_stop = Arc::clone(&stop);
        let writer_stop = Arc::clone(&stop);
        let reader_handler = thread::spawn(move || loop {
            let result = reader_logger.lock().unwrap().read().unwrap();
            println!("{}", result.0);
            if *reader_stop.lock().unwrap() {
                break;
            }
            thread::sleep(Duration::from_secs(1));
        });
        let writer_handler = thread::spawn(move || {
            for i in 0..10 {
                writer_logger
                    .lock()
                    .unwrap()
                    .write(&format!("writing log line {}\n", i))
                    .unwrap();
            }
            thread::sleep(Duration::from_secs(1));
            for i in 10..20 {
                println!("writing log row {}", i);
                writer_logger
                    .lock()
                    .unwrap()
                    .write(&format!("writing log line {}\n", i))
                    .unwrap();
            }
            let mut stop = writer_stop.lock().unwrap();
            *stop = true;
        });
        handlers.push(reader_handler);
        handlers.push(writer_handler);

        handlers
            .into_iter()
            .for_each(|handler| handler.join().unwrap());
        Ok(())
    }
}
