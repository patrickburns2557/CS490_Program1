/**********************************
Patrick Burns
2022-10-08
CS 490 Programming Assignment 1

Purpose: Create Process Node structs based on
the number the user specifies, and store them
in a queue to print them out in order of creation,
and store them in binary heap to print out in
order of priority 
**********************************/

/* Use declarations */
use std::{collections::{VecDeque, BinaryHeap}, cmp::Ordering};
use rand::Rng;

/* Structs and their associated Methods */
#[derive(Debug)]
#[derive(Eq)]
#[derive(Clone)]
struct Process {
    priority: u8,
    id: u32,
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
//Implements ordering to be used in the Binary Heap to sort the Process instances by priority
//Reverses the normally expected output of cmp to create a min binary heap
impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //Reverse the Ordering result to create a min binary heap
        match self.priority {
            b if b > other.priority => Ordering::Less,
            b if b < other.priority => Ordering::Greater,
            b if b == other.priority => Ordering::Equal,
            _ => Ordering::Equal
        }
    }
}
//Implements PartialOrd for Process to be used by Ord
impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
//Implements PartialEq f or Process to be used by Ord
impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

fn main() {
    let mut num_processes: u32 = 0;
    let mut process_queue: VecDeque<Process> = VecDeque::new();
    let mut process_binary_heap: BinaryHeap<Process> = BinaryHeap::new();
    
    
    //Generate process nodes and push them onto the queue GET USER INPUT FOR THIS LATER
    for _i in 1..=10 {
        num_processes += 1;
        let p1 = Process::new(num_processes);
        process_queue.push_back(p1);
    }

    //Copy the process nodes from the queue and place the copies into the binary heap
    for i in &process_queue {
        process_binary_heap.push(i.clone());
    }


    for _i in 0..=(num_processes-1) {
        let p = process_binary_heap.pop().unwrap();
        println!("Id: {:>3}, priority: {:>4}, sleep time: {:>4}, description: {}", p.id, p.priority, p.sleep_time, p.description);
    }
    println!("\n\n");
    for _i in 0..=(num_processes-1) {
        let p = process_queue.pop_front().unwrap();
        println!("Id: {:>3}, priority: {:>4}, sleep time: {:>4}, description: {}", p.id, p.priority, p.sleep_time, p.description);
    }
    
}