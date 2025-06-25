use std::io::Write;
use std::time::Instant;
use std::process::{Command, Stdio};
use super::file::{File, PreparationMethod, FileType};

enum RunnerStatus {
    NoStart,
    Done,
}

pub struct Runner {
    file: File,
    input: Option<File>,
    output: Option<String>,
    running_time: Option<u128>,
    runner_status: RunnerStatus
}

pub trait EndMethod {
    fn get_output(&self) -> String;
    fn get_running_time(&self) -> u128;
}

impl Runner {
    pub fn new(file: File, input:Option<File>) -> Self {
        Self {
            file,
            input,
            output: None,
            running_time: None,
            runner_status: RunnerStatus::NoStart,
        }
    }

    pub fn run(&mut self) {
        let start = Instant::now();

        let mut child = match self.file.get_type() {
            FileType::Binary => {
                let child = Command::new(self.file.get_file_path())
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn().unwrap();
                child
            }
            FileType::Python => {
                let child = Command::new(get_default_python())
                    .arg(self.file.get_file_path())
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn().unwrap();
                child
            }
            FileType::Text => {
                panic!("{}", format!("Error!'{}' can't be run!", self.file.get_file_name()))
            }
        };

        let stdin = child.stdin.as_mut().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to open stdin")
        }).unwrap();

        if let Some(file) = &self.input {
            stdin.write_all(file.get_content().unwrap().as_bytes()).unwrap()
        }

        let output = child.wait_with_output().unwrap();

        self.running_time = Some(start.elapsed().as_millis());

        self.output = Some(String::from_utf8_lossy(&output.stdout).to_string());
        self.runner_status = RunnerStatus::Done;
    }
}

#[cfg(unix)]
fn get_default_python() -> String {
    "python3".to_string()
}

#[cfg(windows)]
fn get_default_python() -> String {
    "python".to_string()
}

impl EndMethod for Runner {
    fn get_output(&self) -> String {
        if let Some(out) = self.output.clone() {
            out
        } else {
            "".to_string()
        }
    }

    fn get_running_time(&self) -> u128 {
        if let Some(running_time) = self.running_time.clone() {
            running_time
        } else {
            panic!("Can't get running time!")
        }
    }
}