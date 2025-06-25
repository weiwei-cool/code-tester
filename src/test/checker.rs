use super::runner::{EndMethod, Runner};

pub struct Checker {
    ans: Option<String>,
    time_limit: u128,
}

pub enum CheckerStatus {
    UnCheck,
    True,
    False,
    TimeLimitError
}

impl Checker {
    pub fn new(ans: Option<String>, time_limit: Option<u128>) -> Self{
        match time_limit {
            None => {
                Self {
                    ans,
                    time_limit: 1000,
                }
            }
            Some(time_limit) => {
                Self {
                    ans,
                    time_limit,
                }
            }
        }
    }

    pub fn check(&self, runner: &Runner) -> CheckerStatus {
        let running_time = runner.get_running_time();
        if let Some(ans) = self.ans.clone() {
            let output = runner.get_output();
            if ans.trim() == output.trim() {
                if running_time > self.time_limit {
                    CheckerStatus::TimeLimitError
                } else {
                    CheckerStatus::True
                }
            } else {
                CheckerStatus::False
            }
        } else {
            if running_time > self.time_limit {
                CheckerStatus::TimeLimitError
            } else {
                CheckerStatus::UnCheck
            }
        }
    }
}