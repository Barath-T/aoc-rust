use regex::Regex;
use std::fs;

pub struct Solve {
    pub input: String,
}

impl Solve {
    pub fn parse(&mut self) {
        self.input = fs::read_to_string("input/day3.txt").expect("file not found");
    }

    pub fn uncorrupted_mul(&self) -> u32 {
        let mut result = 0;
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        re.captures_iter(&self.input).for_each(|c| {
            let (_, [first, second]) = c.extract::<2>();
            result += first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap();
        });

        return result;
    }

    pub fn conditional_mul(&self) -> u32 {
        let mut result = 0;
        let mut on = true;

        let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|don't\(\)|do\(\)").unwrap();
        re.find_iter(&self.input).for_each(|m| {
            let mat = m.as_str();
            match mat {
                "don't()" => on = false,
                "do()" => on = true,
                mul => {
                    if on {
                        let digit_re = Regex::new(r"\d{1,3}").unwrap();
                        let first_mat = digit_re.find(mul).unwrap();
                        let first = first_mat.as_str().parse::<u32>().unwrap();
                        let second = digit_re
                            .find_at(mul, first_mat.end())
                            .unwrap()
                            .as_str()
                            .parse::<u32>()
                            .unwrap();
                        result += first * second;
                    }
                }
            }
        });

        return result;
    }
}
