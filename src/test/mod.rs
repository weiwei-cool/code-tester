use std::io::{stdin, Read};
use std::path::Path;
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
    ans_file: Option<String>,
    time_limit: Option<u128>,
    data: Option<String>,
    no_input: bool,
}

#[derive(Clone)]
struct TestPoint {
    status: CheckerStatus,
    running_time: u128,
}

enum IoMethod {
    NormalIo,
    FolderMultipleIo,
    ZipMultipleIo,
}

impl Test {
    pub fn new(file: &String,
               input_file: &Option<String>,
               out_file: &Option<String>,
               time_limit: &Option<u128>,
               data: &Option<String>,
               no_input: &bool) -> Self {
        Self {
            file: file.clone(),
            input_file: input_file.clone(),
            ans_file: out_file.clone(),
            time_limit: time_limit.clone(),
            data: data.clone(),
            no_input: no_input.clone(),
        }
    }

    pub fn run(&mut self){
        if !self.data.is_some() {
            self.run_normal()
        } else {
            let path = self.data.clone().unwrap();
            let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap().to_string();
            let status = {
                let in_path = format!("{}.in", path);
                let ans_path = format!("{}.ans", path);
                let (i, o) = (
                    Path::new(&in_path), Path::new(&ans_path)
                );
                if i.exists() && o.exists() {
                    IoMethod::NormalIo
                } else {
                    let in_path = format!("{}/{}1.in", path, file_name);
                    let ans_path = format!("{}/{}1.ans", path, file_name);
                    let i = Path::new(&in_path);
                    let o = Path::new(&ans_path);
                    if i.exists() && o.exists() {
                        IoMethod::FolderMultipleIo
                    } else {
                        let file_path = format!("{}.zip", path);
                        let file_path_o = format!("{}", path);
                        let file = Path::new(&file_path);
                        let file_o = Path::new(&file_path_o);
                        if file.exists() || (file_o.exists() && path[path.len()-4..] == ".zip".to_string()) {
                            IoMethod::ZipMultipleIo
                        } else {
                            panic!("Cannot parse the IoMethod!");
                        }
                    }

                }
            };

            match status {
                IoMethod::NormalIo => {
                    self.input_file = Some(format!("{}.in", path));
                    self.ans_file = Some(format!("{}.ans", path));
                    self.run_normal();
                }
                IoMethod::FolderMultipleIo => {
                    let mut result_status:Vec<TestPoint> = Vec::new();
                    let mut running_time:u128 = 0;
                    let mut counter: u8 = 0;
                    let mut score: u8 = 0;
                    loop {
                        let in_path = format!("{}/{}{}.in", path, file_name, counter + 1);
                        let ans_path = format!("{}/{}{}.ans", path, file_name, counter + 1);
                        let i = Path::new(&in_path);
                        let o = Path::new(&ans_path);
                        if !(i.exists() && o.exists()) {
                            break;
                        }
                        self.input_file = Some(in_path);
                        self.ans_file = Some(ans_path);

                        let result = self.run_multiple(None, None);
                        result_status.push(result.clone());

                        match &result.status {
                            CheckerStatus::True => {
                                score += 1
                            }
                            _ => {}
                        }

                        running_time += result.running_time;
                        counter += 1;
                    }

                    print_multiple_result(score, counter, running_time, result_status);

                }
                IoMethod::ZipMultipleIo => {
                    let mut map = std::collections::HashMap::new();
                    let path = self.data.clone().unwrap();
                    let (zip_path, file_name) = if path.len() <= 4 {
                        (format!("{}.zip", path), path)
                    } else if path[path.len()-4..] == ".zip".to_string() {
                        ((&path).clone(), path[..path.len()-4].to_string())
                    } else {
                        (format!("{}.zip", path), path)
                    };

                    let file = std::fs::File::open(&zip_path).unwrap();
                    let mut reader = std::io::BufReader::new(file.try_clone().unwrap());

                    let mut archive = zip::ZipArchive::new(&mut reader).unwrap();
                    for i in 0..archive.len() {
                        let name = archive.by_index(i).ok().map(|f| f.name().to_string());
                        if let Some(name) = name {
                            map.insert(name, i);
                        }
                    }

                    let mut result_status:Vec<TestPoint> = Vec::new();
                    let mut running_time:u128 = 0;
                    let mut counter: u8 = 0;
                    let mut score: u8 = 0;
                    loop {
                        let in_path = format!("{}{}.in", file_name, counter + 1);
                        let ans_path = format!("{}{}.ans", file_name, counter + 1);

                        let (input_file_content, ans_file_content) = if let (
                            Some(_input),
                            Some(_ans)) = (map.get(&in_path), map.get(&ans_path)) {

                            let input_file = read_zip_file(&zip_path, &*in_path).unwrap().trim().to_string();
                            let ans_file = read_zip_file(&zip_path, &*ans_path).unwrap().trim().to_string();

                            (input_file, ans_file)
                        } else {
                            break;
                        };

                        let result = self.run_multiple(Option::from(input_file_content)
                                                       , Option::from(ans_file_content));
                        result_status.push(result.clone());

                        match &result.status {
                            CheckerStatus::True => {
                                score += 1
                            }
                            _ => {}
                        }

                        running_time += result.running_time;
                        counter += 1
                    }

                    print_multiple_result(score, counter, running_time, result_status);

                }
            }
        }
    }

    fn run_multiple(&self, input_file_content:Option<String>, ans_file_content:Option<String>) -> TestPoint{
        let runner = self.run_program(input_file_content);

        let checker = match ans_file_content {
            None => {
                let ans_file = File::new(&(&self.ans_file).clone().unwrap(), FileType::Text);
                Checker::new(Some(ans_file.get_content().unwrap()), self.time_limit)
            }
            Some(ans) => {
                Checker::new(Some(ans), self.time_limit)
            }
        };

        let status = checker.check(&runner);

        match status {
            CheckerStatus::UnCheck => {
                panic!("Multiple test groups must have answers!")
            }
            _ => {
                TestPoint {
                    status,
                    running_time: runner.get_running_time(),
                }
            }
        }
    }

    fn run_normal(&self) {
        let runner = self.run_program(None);

        let ans_file = match &self.ans_file {
            None => {
                None
            }
            Some(ans_file_path) => {
                let ans_file = File::new(&ans_file_path, FileType::Text);
                Option::from(ans_file.get_content().unwrap())
            }
        };

        let checker = Checker::new(ans_file, self.time_limit);

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
                println!("Result: {}", "TimeLimitError".blue());
                println!("Running time: {}ms", runner.get_running_time().to_string().green());
            }
        }
    }

    fn run_program(&self, input_file_content:Option<String>) -> Runner {
        let python_flag = if self.file.len() > 3 {
            &self.file[self.file.len()-3..] == ".py"
        } else {
            false
        };
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
            } else if let Some(input_file_content) = &input_file_content {
                Runner::new(running_file, Option::from(File::form_string(input_file_content.to_string())))
            } else {
                println!(include_str!("text/input_format"));
                let mut input = String::new();

                stdin().read_to_string(&mut input).expect("Input error!");

                Runner::new(running_file, Option::from(File::form_string(input)))
            }
        };

        runner.run();

        runner
    }
}

fn print_multiple_result(score:u8, counter: u8, running_time:u128, result_status:Vec<TestPoint>) {
    let rate = ((score as f64 / counter as f64) * 100f64) as u8;
    if score == counter {
        println!("Result: {}", "Accepted".green());
        println!("Rate: {}", rate.to_string().green());
        println!("Running time: {}ms", running_time.to_string().green());
    } else {
        println!("Result: {}", "Unaccepted".red());
        let rate = {
            if rate >= 50 {
                rate.to_string().bright_yellow()
            } else if rate >= 20 {
                rate.to_string().bright_magenta()
            } else {
                rate.to_string().red()
            }
        };
        println!("Rate: {}", rate);
        println!("Running time: {}ms", running_time.to_string().green());
        println!("Unaccepted test points:");
        for i in 0..result_status.len() {
            match result_status[i].status {
                CheckerStatus::False => {
                    println!("Test {}:", i+1);
                    println!("  Result: {}", "Wrong Answer".red());
                    println!("  Running time: {}ms", result_status[i]
                        .running_time.to_string().green());
                }
                CheckerStatus::TimeLimitError => {
                    println!("Test {}:", i+1);
                    println!("  Result: {}", "TimeLimitError".blue());
                    println!("  Running time: {}ms", result_status[i]
                        .running_time.to_string().green());
                }
                _ => {}
            }
        }
    }
}

fn read_zip_file(zip_path: &str, file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let mut target_file = archive.by_name(file_name)?;
    let mut contents = String::new();
    target_file.read_to_string(&mut contents)?;

    Ok(contents)
}