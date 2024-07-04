use std::fs::{self, File};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use chrono::Local;
use log::LevelFilter;
use fern::Dispatch;
use std::time::{Duration, SystemTime};


pub struct Logger {
    level: LevelFilter,
    file: Option<PathBuf>,
    started: bool,
}

impl Logger {
    pub fn new(level: LevelFilter) -> Self {
        Logger {
            level,
            file: None,
            started: false,
        }
    }

    pub fn start(&mut self) -> Result<(), Error> {
        if self.started {
            return Ok(());
        }

        self.clean_logs()?;

        let logs_dir = "logs";
        fs::create_dir_all(logs_dir).map_err(|e| {
            Error::new(ErrorKind::Other, e)
        })?;

        let log_file = PathBuf::from(format!(
            "{}/{}.log",
            logs_dir,
            Local::now().format("%Y-%m-%d_%H-%M-%S")
        ));

        match Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[ORACLE][{}][{}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    message
                ))
            })
            .level(self.level)
            .chain(std::io::stdout())
            .chain(File::create(&log_file).map_err(|e| {
                Error::new(ErrorKind::Other, e)
            })?)
            .filter(|metadata| metadata.target().contains("oracle"))
            .apply() {
                Ok(dispatcher) => dispatcher,
                Err(e) => return Err(Error::new(ErrorKind::Other, e)),
            };

        self.file = Some(log_file);
        self.started = true;

        Ok(())
    }

    pub fn clean_logs(&self) -> Result<(), Error> {
        let logs_dir = "logs";
        let max_age = Duration::from_secs(10 * 24 * 60 * 60);
        let max_size = 100 * 1024 * 1024; 
        let mut total_size = 0;
        let mut oldest_file: Option<(PathBuf, SystemTime)> = None;

        for entry in fs::read_dir(logs_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let metadata = fs::metadata(&path)?;
                let modified_time = metadata.modified()?;
                let age = SystemTime::now().duration_since(modified_time)
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;

                total_size += metadata.len();

                if age >= max_age {
                    fs::remove_file(&path)
                        .map_err(|e| Error::new(ErrorKind::Other, e))?;
                    if self.started {
                        log::info!("Removed log file: {:?}", path);
                    } else { println!("Removed log file: {:?}", path); }


                } else if oldest_file.is_none() || modified_time < oldest_file.as_ref().unwrap().1 {
                    oldest_file = Some((path, modified_time));
                }
            }
        }

        if total_size >= max_size {
            if let Some((oldest_path, _)) = oldest_file {
                fs::remove_file(&oldest_path)
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;
                if self.started {
                    log::info!("Removed log file: {:?}", oldest_path);
                } else { println!("Removed log file: {:?}", oldest_path); };
            }
        }

        Ok(())
    }
}

