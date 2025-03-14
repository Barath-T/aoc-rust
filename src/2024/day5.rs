use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
pub struct Solve {
    pub order_map: HashMap<u32, HashSet<u32>>,
    pub updates: Vec<Vec<u32>>,
}

impl Solve {
    pub fn parse(&mut self) {
        let input: String = fs::read_to_string("input/day5.txt").expect("input file error!");
        let mut parse_order: bool = true;
        for line in input.lines() {
            if line == "" {
                parse_order = false;
                continue;
            }
            if parse_order {
                let order: Vec<u32> = line.split('|').map(|e| e.parse::<u32>().unwrap()).collect();
                self.order_map
                    .entry(order[0])
                    .and_modify(|entry| {
                        entry.insert(order[1]);
                    })
                    .or_insert(HashSet::new());
                self.order_map.entry(order[0]).and_modify(|entry| {
                    entry.insert(order[1]);
                });
            } else {
                self.updates.push(
                    line.split(',')
                        .map(|e| e.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                );
            }
        }
    }
    pub fn valid(&self, update: &Vec<u32>) -> bool {
        let mut flag: bool = true;
        let mut check_set: HashSet<u32> = HashSet::new();
        println!("vecs: {:?}", update);
        for page in update {
            for rule in self.order_map.get(&page).unwrap_or(&HashSet::new()) {
                if check_set.contains(&rule) {
                    flag = false;
                    break;
                }
            }
            if flag {
                check_set.insert(*page);
            } else {
                break;
            }
        }

        return flag;
    }

    pub fn part2(&self) -> u32 {
        let mut result: u32 = 0;
        let mut record = self.updates.clone();
        for update in &mut record {
            if self.valid(update) {
                continue;
            }
            let len = update.len();
            let mut idx = len-1;
            loop {
                let mut mark: Option<usize> = None;
                let rule: &HashSet<u32> = match self.order_map.get(&update[idx]){
                    Some(r) => r,
                    None=>{
                        idx -= 1;
                        continue;
                    }
                };
                for j in 1..idx+1 {
                    println!("{}, {}", idx, idx-j);
                    // len-1-i-j
                    if rule.contains(&update[idx-j]) {
                        mark = Some(idx-j);
                    }

                }
                match mark {
                    Some(m) => {
                        let temp = update[m];
                        update[m] = update[idx];
                        update[idx] = temp;
                    },
                    None => idx -= 1
                }
                if idx == 0 {
                    break;
                }
                println!("{:?}", update);
                //if idx == len-2 {
                //    break;
                //}
            }
            result += update[update.len()/2];
        }
        return result;
    }
}
