use std::fs;

pub struct Solve {
    pub reports: Vec<Vec<u32>>,
}

impl Solve {
    pub fn parse(&mut self) {
        fs::read_to_string("input/day2.txt")
            .unwrap()
            .trim()
            .lines()
            .for_each(|line| {
                self.reports.push(
                    line.split(' ')
                        .map(|x| {
                            let smth: u32 = x.parse().expect("invalid input");
                            smth
                        })
                        .collect(),
                );
            });
    }
    fn is_safe_report(report: &Vec<u32>) -> bool {
        let mut prev = report[0];
        let inc_flag = report[0] < report[1];
        for i in 1..report.len() {
            if (inc_flag && prev > report[i])
                || (!inc_flag && prev < report[i])
                || prev == report[i]
            {
                return false;
            }
            if prev.abs_diff(report[i]) > 3 {
                return false;
            }
            prev = report[i];
        }
        return true;
    }
    pub fn count_safe_reports(&self) -> u32 {
        let mut result = 0;

        for report in &self.reports {
            if Solve::is_safe_report(&report) {
                result += 1;
            }
        }
        return result;
    }

    pub fn count_safe_reports_damped(&self) -> u32 {
        let mut result = 0;

        for report in &self.reports {
            if Solve::is_safe_report(&report) {
                result += 1;
            } else {
                for i in 0..report.len() {
                    let changed: Vec<u32> = report
                        .into_iter()
                        .enumerate()
                        .filter(|(j, _)| i != *j)
                        .map(|(_, data)| *data)
                        .collect();
                    if Solve::is_safe_report(&changed) {
                        result += 1;
                        break;
                    }
                }
            }
        }
        return result;
    }
}
