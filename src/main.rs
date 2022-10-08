use std::collections::{VecDeque, BinaryHeap};

use rand::Rng;

/* Structs */
#[derive(Debug)]
#[derive(Ord)]
#[derive(PartialOrd)]
#[derive(PartialEq)]
#[derive(Eq)]
struct Process {
    id: u32,
    priority: u8,
    sleep_time: u8,
    description: String
}
impl Process {
    //associated function to return a new Process instance
    fn new(id: u32) -> Self {
        //Creating the string to store in the description field
        let mut desc = String::from("Process Node: ");
        desc.push_str(&id.to_string());
        
        Self {
            id,
            priority: rand::thread_rng().gen_range(1..=100),
            sleep_time: rand::thread_rng().gen_range(100..=200),
            description: desc
        }
    }
}



fn main() {
    let mut num_processes: u32 = 0;
    let mut process_queue: VecDeque<Process> = VecDeque::new();
    let mut process_binary_heap: BinaryHeap<Process> = BinaryHeap::new();
    
    
    for _i in 1..=100 {
        num_processes += 1;
        let p1 = Process::new(num_processes);
        //process_queue.push_back(p1);
        process_binary_heap.push(p1);
    }
    


    for i in 0..=(num_processes-1) {
        let p = process_binary_heap.pop();
        println!("{:?}", p);
    }
    
}