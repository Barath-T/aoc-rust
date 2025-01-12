use std::fs;

pub struct Solve {
    pub lid: Vec<u32>,
    pub _lid: Vec<u32>,
}

impl Solve {
    pub fn parse(&mut self) {
        fs::read_to_string("input/day1.txt")
            .unwrap()
            .trim()
            .lines()
            .for_each(|x| {
                let (left, right) = x.split_once(' ').unwrap();
                self.lid
                    .push(left.trim().parse().expect("enter only valid number"));
                self._lid
                    .push(right.trim().parse().expect("enter only valid number"));
            });

        self.lid.sort();
        self._lid.sort();
    }

    pub fn distance(&self) -> u32 {
        let mut result = 0;

        for (i, value) in self.lid.iter().enumerate() {
            result += value.abs_diff(self._lid[i]);
        }

        return result;
    }
    pub fn similarity_score(&self) -> u32 {
        let mut result = 0;
        let mut i = 0;
        for value in &self.lid {
            let mut count = 0;

            for idx in i..self.lid.len() {
                if self._lid[idx] > *value {
                    result += value * count;
                    break;
                } else if self._lid[idx] == *value {
                    count += 1;
                }
                i += 1;
            }
        }

        return result;
    }
}
