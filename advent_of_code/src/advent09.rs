use std::collections::{BTreeMap};
use crate::utils::{Solve, Label, assert_display};

#[derive(Debug, Clone, Copy)]
struct Block{
    file_id: Option<usize>,
    size: usize
}

impl Block{
    fn new(file_id: Option<usize>, size: usize)->Self{
        Self {
            file_id,
            size
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    disk: BTreeMap<usize,usize>,
    disk_alt: BTreeMap<usize, Block>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(9),
            disk: BTreeMap::new(),
            disk_alt: BTreeMap::new(),
        }
    }
}

fn search_block_to_write(disk: &BTreeMap<usize, Block>, current_position: usize) -> Option<usize>{
    if let Some(block) =disk.get(&current_position) {
        for (&test_position, &test_block) in disk.iter() {
            if test_position>=current_position{
                return None;
            }
            if let None = test_block.file_id {
                let d = test_block.size as isize - block.size as isize;
                if d >= 0 {
                    return Some(test_position);
                }
            }
        }
    }
    None
}

fn move_block(disk: &mut BTreeMap<usize, Block>, source_position: usize, target_position: usize) {
    if let Some(source_block) = disk.remove(&source_position) {
        if let Some(target_block) = disk.get_mut(&target_position) {
            let d = target_block.size as isize - source_block.size as isize;

            target_block.file_id = source_block.file_id;
            target_block.size = source_block.size;

            if d > 0 {
                let add_position = target_position + target_block.size;
                let b = Block::new(None, d as usize);
                disk.insert(add_position, b);
            }
            let b = Block::new(None, source_block.size);
            disk.insert(source_position, b);
        }
    }
}


impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        let mut curr_id: usize = 0;
        let mut is_file: bool = true;
        let mut curr_position: usize = 0;
        for ch in line.chars(){
            match ch.to_digit(10).map(|d| d as usize){
                Some(num) =>{
                    if is_file {
                        self.disk_alt.insert(curr_position, Block::new(Some(curr_id), num));
                        for _ in 0..num {
                            self.disk.insert(curr_position, curr_id);
                            curr_position+=1;
                        }
                    }
                    else{
                        self.disk_alt.insert(curr_position, Block::new(None, num));
                        curr_position+=num;
                        curr_id+=1;
                    }
                    is_file = !is_file;
                }
                None => { "invalid".parse::<i32>()?; }
            }
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        if let Some((k,v)) = self.disk.last_key_value() {
            println!("Space taken {}", k);
            println!("Number of files {}", v);
        }
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut disk = self.disk.clone();
        let mut curr_position: usize = 0;

        loop{
            while let Some(_) = disk.get(&curr_position){
                curr_position+=1
            }
            if let Some((&file_position, &file_id)) = disk.last_key_value(){
                if file_position<curr_position{
                    break;
                }
                disk.remove(&file_position);
                disk.insert(curr_position, file_id);
                curr_position+=1;
            }
        }
        let mut checksum = 0;
        for (file_position, file_id) in disk{
            checksum+=file_position*file_id;
        }
        assert_display(checksum, Some(1928), 6259790630969, "Checksum", test_mode)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut disk = self.disk_alt.clone();

        let mut files: BTreeMap<usize, usize> = BTreeMap::new();
        for (&current_position, block) in disk.iter(){
            if let Some(file_id) = block.file_id{
                files.insert(file_id, current_position);
            }
        }
        for (_, source_position) in files.into_iter().rev(){
            let target_position = search_block_to_write(&disk, source_position);
            if let Some(target_position) = target_position{
                move_block(&mut disk, source_position, target_position);
            }
        }
        let mut checksum = 0;
        for (current_position, block) in &disk{
            if let Some(file_id) = block.file_id {
                checksum+=file_id*((2*current_position+block.size-1)*block.size)/2;
            }
        }
        assert_display(checksum, Some(2858), 6289564433984, "Checksum", test_mode)
    }
}

