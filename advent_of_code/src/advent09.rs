use std::collections::{BTreeMap, BTreeSet};
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
            Some(Chunk::new(self.file_id, remaining_size))
        } else {
            None
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    disk_with_chunks: BTreeMap<usize, Chunk>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(9),
            disk_with_chunks: BTreeMap::new(), //positions of chunks (free or occupied)
        }
    }
}

impl Advent{
    fn solve(&self,
             no_split: bool,
             result_test: usize,
             result_prd: usize,
             test_mode: bool,
             part: u8
    ) -> Result<String, String>{
        self.check_input(Some(part))?;
        let mut disk = self.disk_with_chunks.clone();

        let files: BTreeMap<usize, usize> = disk
            .iter()
            .filter_map(|(&current_position, block)| block.file_id.map(|file_id| (file_id, current_position)))
            .collect();

        let mut free_chunks: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        for (&position, chunk) in disk.iter(){
            if chunk.file_id.is_none(){
                free_chunks.entry(chunk.size).or_insert_with(BTreeSet::new).insert(position);
            }
        }

        let mut checksum = 0;
        for (_, source_position) in files.into_iter().rev(){
            loop {
                let source_block = disk.get(&source_position).unwrap();

                if source_block.file_id.is_none(){
                    break;
                }
                let s = if no_split { source_block.size } else { 0 };

                let target_positions: Vec<&usize> = free_chunks
                    .range(s..)
                    .filter_map(|(_, v)| v.first())
                    .filter(|&&p| p < source_position)
                    .collect();

                if let Some(&&target_position) = target_positions.iter().min() {
                    if let (Some(mut source_block), Some(mut target_block)) = (
                        disk.remove(&source_position),
                        disk.remove(&target_position)
                    ) {
                        free_chunks.get_mut(&target_block.size).unwrap().remove(&target_position);

                        if target_block.size >= source_block.size {
                            // Target has enough space for the source block
                            if let Some(remainder) = target_block.reserve_memory(&source_block) {
                                let add_position = target_position + target_block.size;
                                free_chunks.entry(remainder.size).or_insert_with(BTreeSet::new).insert(add_position);
                                disk.insert(add_position, remainder);
                            }
                            disk.insert(source_position, target_block);
                        } else {
                            // Source block needs to be split
                            if let Some(remainder) = source_block.reserve_memory(&target_block) {
                                let split_position = source_position + remainder.size;
                                disk.insert(source_position, remainder);
                                disk.insert(split_position, target_block);
                            }else{
                                disk.insert(source_position, target_block);
                            }
                        }

                        checksum += source_block.checksum_increment(target_position);
                        disk.insert(target_position, source_block);
                    }
                } else {
                    checksum += source_block.checksum_increment(source_position);
                    break;
                }
            }
        }
        assert_display(checksum, Some(result_test), result_prd, "Checksum", test_mode)
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
                    }
                    else{
                        self.disk_with_chunks.insert(curr_position, Chunk::new(None, num));
                        curr_id+=1;
                    }
                    curr_position+=num;
                    is_file = !is_file;
                }
                None => { "invalid".parse::<i32>()?; }
            }
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Number of chunks {}", self.disk_with_chunks.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.solve(false, 1928, 6259790630969, test_mode, 1)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.solve(true, 2858, 6289564433984, test_mode, 2)
    }
}

