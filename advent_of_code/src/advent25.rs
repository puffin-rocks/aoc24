use std::collections::HashSet;
use crate::utils::{Solve, Label, read_lines, assert_display};

pub(crate) struct Advent {
    label: Label,
    locks: HashSet<[u8;5]>,
    keys: HashSet<[u8;5]>,
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(25),
            locks: HashSet::new(),
            keys: HashSet::new()
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn read_input(&mut self, test_mode: bool) -> Result<(), std::num::ParseIntError>{
        let filename = self.get_label().get_puzzle_input_path(test_mode);

        if let Ok(lines) = read_lines(filename) {
            let mut is_lock: Option<bool> = None;
            let mut entry: [u8;5] = [0; 5];
            for line in lines.flatten() {
                if line.is_empty() {
                    if let Some(is_lock) = is_lock {
                        if is_lock {
                            self.locks.insert(entry.clone());
                        } else {
                            self.keys.insert(entry.clone());
                        }
                    }
                    is_lock = None;
                } else {
                    match is_lock {
                        None => {
                            if line.starts_with('#') {
                                entry = [0; 5];
                                is_lock = Some(true);
                            } else {
                                entry = [5; 5];
                                is_lock = Some(false);
                            }
                        }
                        Some(is_lock) => {
                            for (v, ch) in entry.iter_mut().zip(line.chars()) {
                                if is_lock {
                                    if ch == '#' {
                                        *v += 1;
                                    }
                                } else if ch == '.' {
                                    *v -= 1;
                                }
                            }
                        }
                    }
                }
            }
            if let Some(is_lock) = is_lock {
                if is_lock {
                    self.locks.insert(entry.clone());
                } else {
                    self.keys.insert(entry.clone());
                }
            }
            self.get_label_mut().has_input = true;
        }
        Ok(())
    }
    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Number of locks: {}", self.locks.len());
        println!("Number of keys: {}", self.keys.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut cnt = 0;
        for lock in &self.locks {
            for key in &self.keys {
                if lock.iter().zip(key).all(|(l, k)| l + k <= 5) {
                    cnt += 1;
                }
            }
        }
        assert_display(cnt, None, 3320, "Nummber of non-overlapping pairs", test_mode)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        Ok(String::from("No computation required"))
    }
}