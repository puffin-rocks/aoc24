use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::rc::Rc;
use itertools::Itertools;
use crate::hashset;
use crate::utils::{Solve, Label};
use crate::utils::*;

pub(crate) struct Advent {
    label: Label,
    edges: Vec<HashSet<Rc<String>>>,
    vertices: HashSet<Rc<String>>,
    d_edges: HashSet<(Rc<String>, Rc<String>)>
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(23),
            edges: Vec::new(),
            vertices: HashSet::new(),
            d_edges: HashSet::new()
        }
    }
}

impl Advent{
    #[allow(dead_code)]
    fn is_disjoint(&self) -> bool{
        let mut neighbours: VecDeque<Rc<String>> = VecDeque::new();
        let mut undecided: Vec<Rc<String>> = self.vertices.iter().cloned().collect();
        if let Some(el) = self.vertices.iter().next(){
            neighbours.push_back(el.clone());
        }
        while let Some(v) = neighbours.pop_front(){
            let mut undecided_next: Vec<Rc<String>> = Vec::new();
            while let Some(v_other) = undecided.pop(){
                if self.d_edges.contains(&(v.clone(), v_other.clone())){
                    neighbours.push_back(v_other);
                }
                else{
                    undecided_next.push(v_other);
                }
            }
            undecided = undecided_next;
            if undecided.is_empty(){
                break
            }
        }
        undecided.len()!=0
    }
    fn get_connected_triplets(&self)->HashSet<BTreeSet<Rc<String>>>{
        let mut q: HashSet<BTreeSet<Rc<String>>> = HashSet::new();
        for v in self.vertices.iter(){
            for e in self.edges.iter(){
                if !e.contains(v) &&
                    e.iter().all( |v_other|
                        self.d_edges.contains(&(v.clone(), v_other.clone()))){
                    let mut vec:BTreeSet<Rc<String>> = e.iter().cloned().collect();
                    vec.insert(v.clone());
                    q.insert(vec);
                }
            }
        }
        q
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        if let Some((el1, el2)) = line.split_once("-"){
            let (el1, el2) = (el1.to_string(), el2.to_string());
            let (el1, el2) = match (self.vertices.get(&el1), self.vertices.get(&el2)){
                (Some(el1), Some(el2)) => (el1.clone(), el2.clone()),
                (Some(el1), None) => {
                    let el2 = Rc::new(el2);
                    (el1.clone(), el2)
                },
                (None, Some(el2)) => {
                    let el1 = Rc::new(el1);
                    (el1, el2.clone())
                },
                (None, None) => {
                    let el1 = Rc::new(el1);
                    let el2 = Rc::new(el2);
                    (el1, el2)
                }
            };
            self.vertices.insert(el1.clone());
            self.vertices.insert(el2.clone());
            self.d_edges.insert((el1.clone(), el2.clone()));
            self.d_edges.insert((el2.clone(), el1.clone()));
            self.edges.push(hashset!(el1, el2));
        }else{
            "invalid".parse::<i32>()?;
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Number of edges: {}", self.edges.len());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let cnt: usize = self.get_connected_triplets().iter().filter(
            |&set|{
                for e in set.iter(){
                    if e.starts_with('t'){
                        return true;
                    }
                }
                false
            }
        ).collect::<Vec<_>>().len();
        assert_display(cnt, Some(7), 1238, "Number of sets", test_mode)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut neighbours: HashMap<Rc<String>, HashSet<Rc<String>>> = HashMap::new();
        for v in self.vertices.iter(){
            for pair in self.d_edges.iter(){
                if &(*pair).0 == v{
                    neighbours.entry(v.clone()).or_insert_with(HashSet::new).insert((*pair).1.clone());
                }
            }
        }

        let mut q = self.get_connected_triplets();
        loop{
            let mut q_next: HashSet<BTreeSet<Rc<String>>> = HashSet::new();
            for e in q.iter() {
                let ns = e.iter().skip(1).map(|v|{
                    neighbours.get(v).unwrap()
                }).collect::<Vec<_>>();
                for v in neighbours.get(e.first().unwrap()).unwrap().iter(){
                    if ns.iter().all(|&n| {
                        n.contains(v)
                    }){
                        let mut s = e.clone();
                        s.insert(v.clone());
                        q_next.insert(s);
                    }
                }
            }
            if q_next.is_empty(){
                break;
            }
            else{
                println!("{:?}", &q_next.len());
                q=q_next;
            }
        }
        if q.len() == 1{
            let result = vec2line(q.iter().next().unwrap().iter().collect_vec());
            assert_display(result,
                           Some(String::from("co,de,ka,ta")),
                           String::from("bg,bl,ch,fn,fv,gd,jn,kk,lk,pv,rr,tb,vw"),
                           "Password",
                           test_mode
            )
        }else{
            Err(String::from("Multiple solutions found"))
        }
    }
}