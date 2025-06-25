use std::io::{stdin, Read};
use colored::Colorize;
use crate::test::checker::{Checker, CheckerStatus};
use crate::test::file::{File, FileType, PreparationMethod};
use crate::test::runner::{EndMethod, Runner};

pub mod file;
mod runner;
mod checker;

pub struct Test {
    file: String,
    input_file: Option<String>,
    out_file: Option<String>,
    time_limit: Option<u128>,
    no_input: bool,
}

impl Test {
    pub fn new(file: &String,
               input_file: &Option<String>,
               out_file: &Option<String>,
               time_limit: &Option<u128>,
               no_input: &bool) -> Self {
        Self {
            file: file.clone(),
            input_file: input_file.clone(),
            out_file: out_file.clone(),
            time_limit: time_limit.clone(),
            no_input: no_input.clone(),
        }
    }

    pub fn run(&self) {
        let python_flag = &self.file[self.file.len()-3..] == ".py";
        let running_file = if python_flag {
            File::new(&self.file, FileType::Python)
        } else {
            File::new(&self.file, FileType::Binary)
        };

        let mut runner = if self.no_input {
            Runner::new(running_file, None)
        } else {
            if let Some(input_file) = &self.input_file {
                Runner::new(running_file, Option::from(File::new(input_file, FileType::Text)))
            } else {
                println!(include_str!("text/input_format"));
                let mut input = String::new();

                stdin().read_to_string(&mut input).expect("Input error!");

                Runner::new(running_file, Option::from(File::form_string(input)))
            }
        };

        runner.run();

        let out_file = match &self.out_file {
            None => {
                None
            }
            Some(out_file_path) => {
                let out_file = File::new(&out_file_path, FileType::Text);
                Option::from(out_file.get_content().unwrap())
            }
        };

        let checker = Checker::new(out_file, self.time_limit);

        let status = checker.check(&runner);

        match status {
            CheckerStatus::UnCheck => {
                println!("Result: {}", "Uncheck".white());
                println!("Running time: {}ms", runner.get_running_time().to_string().green());
                println!("Output: \n{}", runner.get_output().trim());
            }
            CheckerStatus::True => {
                println!("Result: {}", "Accepted".green());
                println!("Running time: {}ms", runner.get_running_time().to_string().green());
            }
            CheckerStatus::False => {
                println!("Result: {}", "Wrong Answer".red());
                println!("Running time: {}ms", runner.get_running_time().to_string().green());
                println!("Output: \n{}", runner.get_output().trim());
            }
            CheckerStatus::TimeLimitError => {
                println!("Result: {}", "TimeLimitError".red());
                println!("Running time: {}ms", runner.get_running_time().to_string().green());
            }
        }
    }
}