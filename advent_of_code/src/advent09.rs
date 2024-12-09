use std::collections::{BTreeMap};
use crate::utils::{Solve, Label, assert_display};

#[derive(Debug, Clone, Copy)]
struct Chunk {
    file_id: Option<usize>,
    size: usize
}

impl Chunk {
    fn new(file_id: Option<usize>, size: usize)->Self{
        Self {
            file_id,
            size
        }
    }

    fn checksum_increment(&self, position: usize) -> usize{
        if let Some(file_id) = self.file_id{
            file_id*((2 * position + self.size-1)*self.size)/2
        }else{
            0
        }
    }

    fn reserve_memory(&mut self, other: &Chunk)->Option<Chunk>{
        let remaining_size = self.size.saturating_sub(other.size);
        self.size = other.size;
        if remaining_size > 0 {
            Some(Chunk::new(None, remaining_size))
        } else {
            None
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    disk_with_blocks: BTreeMap<usize,usize>,
    disk_with_chunks: BTreeMap<usize, Chunk>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(9),
            disk_with_blocks: BTreeMap::new(), //only positions of blocks storing files
            disk_with_chunks: BTreeMap::new(), //positions of chunks (free or occupied)
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
                        self.disk_with_chunks.insert(curr_position, Chunk::new(Some(curr_id), num));
                        for _ in 0..num {
                            self.disk_with_blocks.insert(curr_position, curr_id);
                            curr_position+=1;
                        }
                    }
                    else{
                        self.disk_with_chunks.insert(curr_position, Chunk::new(None, num));
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
        if let Some((k,v)) = self.disk_with_blocks.last_key_value() {
            println!("Number of blocks {}", k);
            println!("Number of files {}", v);
        }
        println!("Number of chunks {}", self.disk_with_chunks.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut disk = self.disk_with_blocks.clone();
        let mut curr_position: usize = 0;
        let mut checksum = 0;
        loop{
            //current position of empty block
            while let Some(file_id) = disk.get(&curr_position){
                checksum+=file_id*curr_position;
                curr_position+=1;
            }
            if let Some((&file_position, &file_id)) = disk.last_key_value(){
                //we do not move files before current empty block
                if file_position<curr_position{
                    break;
                }
                disk.remove(&file_position);
                disk.insert(curr_position, file_id);
                checksum+=file_id*curr_position;
                //empty block search should start from the next item
                curr_position+=1;
            }
        }
        assert_display(checksum, Some(1928), 6259790630969, "Checksum", test_mode)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut disk = self.disk_with_chunks.clone();

        // Use block.file_id.map to:
        //     If block.file_id is Some(file_id), transform it into (file_id, current_position).
        //     If block.file_id is None, exclude the block.
        let files: BTreeMap<usize, usize> = disk
            .iter()
            .filter_map(|(&current_position, block)| block.file_id.map(|file_id| (file_id, current_position)))
            .collect();

        let mut checksum = 0;
        for (_, source_position) in files.into_iter().rev(){

            let mut target_position = None;
            let source_block = disk.get(&source_position).unwrap();
            let mut incr =  source_block.checksum_increment(source_position);
            for (&test_position, &test_block) in disk.iter() {
                if test_position < source_position && test_block.file_id.is_none() && test_block.size >= source_block.size{
                    target_position = Some(test_position);
                    break;
                }
            }

            if let Some(target_position) = target_position {
                if let (Some(source_block), Some(mut target_block)) = (disk.remove(&source_position), disk.remove(&target_position)) {
                    let remainder = target_block.reserve_memory(&source_block);
                    incr = source_block.checksum_increment(target_position);
                    if let Some(remainder) = remainder {
                        let add_position = target_position + target_block.size;
                        disk.insert(add_position, remainder);
                    }
                    disk.insert(target_position, source_block);
                    disk.insert(source_position, target_block);
                }
            }

            checksum+=incr;
        }
        assert_display(checksum, Some(2858), 6289564433984, "Checksum", test_mode)
    }
}