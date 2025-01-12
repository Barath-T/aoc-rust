use std::fs;

pub struct Solve {
    pub input: Vec<String>,
}

impl Solve {
    pub fn parse(&mut self) {
        self.input = fs::read_to_string("input/day4.txt")
            .expect("cant open file")
            .lines()
            .map(|s| s.to_string())
            .collect();

        //println!("{:?}", self.input);
    }

    fn is_xmas(xmas: [char; 4]) -> bool {
        if xmas[0] == 'X' && xmas[1] == 'M' && xmas[2] == 'A' && xmas[3] == 'S' {
            return true;
        }
        return false;
    }
    fn find_around(&self, i: usize, j: usize) -> u32 {
        let mut count = 0;

        if j <= self.input[i].len() - 4 {
            let mut xmas: [char; 4] = ['0', '0', '0', '0'];
            for k in 0..4 {
                xmas[k] = self.input[i]
                    .chars()
                    .nth(j + k)
                    .expect("looking out of string");
            }
            if Self::is_xmas(xmas) {
                count += 1;
            }

            if i >= 3 {
                xmas = ['0', '0', '0', '0'];
                let mut curr_row = i;
                let mut curr_col = j;

                for k in 0..4 {
                    xmas[k] = self.input[curr_row]
                        .chars()
                        .nth(curr_col)
                        .expect("looking out of string");
                    if curr_row >= 1 {
                        curr_row -= 1;
                    }
                    curr_col += 1;
                }
                if Self::is_xmas(xmas) {
                    count += 1;
                }
            }
            if i <= self.input.len() - 4 {
                xmas = ['0', '0', '0', '0'];
                let mut curr_row = i;
                let mut curr_col = j;

                for k in 0..4 {
                    xmas[k] = self.input[curr_row]
                        .chars()
                        .nth(curr_col)
                        .expect("looking out of string");
                    curr_row += 1;
                    curr_col += 1;
                }
                if Self::is_xmas(xmas) {
                    count += 1;
                }
            }
        }
        if j >= 3 {
            let mut xmas: [char; 4] = ['0', '0', '0', '0'];

            for k in 0..4 {
                xmas[k] = self.input[i]
                    .chars()
                    .nth(j - k)
                    .expect("looking out of string");
            }
            if Self::is_xmas(xmas) {
                count += 1;
            }
            if i >= 3 {
                xmas = ['0', '0', '0', '0'];
                let mut curr_row = i;
                let mut curr_col = j;

                for k in 0..4 {
                    xmas[k] = self.input[curr_row]
                        .chars()
                        .nth(curr_col)
                        .expect("looking out of string");
                    if curr_row >= 1 && curr_col >= 1 {
                        curr_row -= 1;
                        curr_col -= 1;
                    }
                }
                if Self::is_xmas(xmas) {
                    count += 1;
                }
            }
            if i <= self.input.len() - 4 {
                xmas = ['0', '0', '0', '0'];
                let mut curr_row = i;
                let mut curr_col = j;

                for k in 0..4 {
                    xmas[k] = self.input[curr_row]
                        .chars()
                        .nth(curr_col)
                        .expect("looking out of string");
                    curr_row += 1;
                    if curr_col >= 1 {
                        curr_col -= 1;
                    }
                }
                if Self::is_xmas(xmas) {
                    count += 1;
                }
            }
        }
        if i >= 3 {
            let mut xmas: [char; 4] = ['0', '0', '0', '0'];

            for k in 0..4 {
                xmas[k] = self.input[i - k]
                    .chars()
                    .nth(j)
                    .expect("looking out of string");
            }
            if Self::is_xmas(xmas) {
                count += 1;
            }
        }
        if i <= self.input.len() - 4 {
            let mut xmas: [char; 4] = ['0', '0', '0', '0'];

            for k in 0..4 {
                xmas[k] = self.input[i + k]
                    .chars()
                    .nth(j)
                    .expect("looking out of string");
            }
            if Self::is_xmas(xmas) {
                count += 1;
            }
        }

        return count;
    }
    pub fn count_xmas(&self) -> u32 {
        let mut count = 0;
        self.input
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(i, s)| {
                for (j, c) in s.chars().enumerate() {
                    if c == 'X' {
                        count += self.find_around(i, j);
                    }
                }
            });
        return count;
    }
    fn is_mas(t: char, d: char) -> bool {
        if (t == 'M' && d == 'S') || (t == 'S' && d == 'M') {
            return true;
        }
        return false;
    }
    fn is_mas_cross(&self, i: usize, j: usize) -> bool {

        if i > 0 && i < self.input.len() - 1 {
            if j > 0 && j < self.input[i].len() - 1 {
                let tl = self.input[i - 1].chars().nth(j - 1).expect("out of range of string");
                let tr = self.input[i - 1].chars().nth(j + 1).expect("out of range of string");
                let dl = self.input[i + 1].chars().nth(j - 1).expect("out of range of string");
                let dr = self.input[i + 1].chars().nth(j + 1).expect("out of range of string");
                if Solve::is_mas(tl, dr) && Solve::is_mas(tr, dl) {
                    return true;
                }
            }
        }
        return false;
    }
    pub fn count_mas_cross(&self) -> u32 {
        let mut count: u32 = 0;

        for i in 1..self.input.len()-1 {
            for (j, c) in self.input[i].chars().enumerate() {
                if c == 'A' {
                    if self.is_mas_cross(i, j) {
                        count += 1;
                    }
                }
            }
        }

        return count;
    }
}
