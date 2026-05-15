use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    // Stores the smaller half of numbers (Max-Heap)
    // The largest element is at the top.
    lowers: BinaryHeap<i32>,  //BinaryHeap is a max-heap
    
    // Stores the larger half of numbers (Min-Heap)
    // The smallest element is at the top.
    uppers: BinaryHeap<Reverse<i32>>, // we user Reverse to make a max-heap into a min-heap
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            lowers: BinaryHeap::new(),
            uppers: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        // Insert into lowers first, then filter the max to uppers
        self.lowers.push(num);
        let max_lower = self.lowers.pop().unwrap();
        self.uppers.push(Reverse(max_lower));

        
        // 'lowers' must absorb the extra element (so lowers > uppers)
        if self.lowers.len() < self.uppers.len() {
            let Reverse(min_upper) = self.uppers.pop().unwrap();
            self.lowers.push(min_upper);
        }
    }

    
    fn find_median(&self) -> i32 {
        *self.lowers.peek().unwrap()
    }
}



fn main() {
    let file = File::open("MedianList.txt").expect("Failed to open file");

    let buf_read = BufReader::new(file);

   let mut median_finder = MedianFinder::new();

   let mut median_vec: Vec<i32> = Vec::new();

    for line in buf_read.lines(){
        let number = line.expect("Failed to load line").parse::<i32>().unwrap();
        median_finder.add_num(number);
        median_vec.push(median_finder.find_median());
    }


    println!("{}",median_vec.into_iter().sum::<i32>() % 10000);


}
