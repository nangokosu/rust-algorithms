use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct Item {
    //id: usize,
    pub value: usize,
    pub weight: usize,
}

// top-down approach

fn knapsack_top_down(
    i: usize,
    w: usize,
    items: &[Item],
    memo: &mut HashMap<(usize, usize), usize>
) -> usize {
    if i == 0 || w == 0 { return 0; }

    // Check hash map cache
    if let Some(&cached_value) = memo.get(&(i, w)) {
        return cached_value;
    }

    let item = &items[i - 1];
    let result = if item.weight > w {
        knapsack_top_down(i - 1, w, items, memo)
    } else {
        std::cmp::max(
            knapsack_top_down(i - 1, w, items, memo),
            item.value + knapsack_top_down(i - 1, w - item.weight, items, memo)
        )
    };

    memo.insert((i, w), result);
    result
}

fn run_knapsack(file_path: &str) -> usize{
    let file_1 = File::open(file_path).unwrap();
    let mut buf_file_1 = BufReader::new(file_1).lines();

    let binding = buf_file_1.next().unwrap().unwrap();
     
    let mut first_line_1 = binding.split_whitespace();
    
    let knapsack_size_1: usize = first_line_1.next().unwrap().parse().unwrap(); // first item

    let item_size_1: usize = first_line_1.next().unwrap().parse().unwrap(); // second item

    let item: Vec<Item> = buf_file_1.map(|line| {
        let line_unwrap = line.unwrap();
        let mut line_split = line_unwrap.split_whitespace();
        let value: usize = line_split.next().unwrap().parse().unwrap();
        let weight: usize = line_split.next().unwrap().parse().unwrap();
        Item{
            value: value,
            weight: weight,
        }
    }).collect();

    let mut memo_table: HashMap<(usize, usize), usize> = HashMap::new();


    let max_value = knapsack_top_down(item_size_1, knapsack_size_1, &item, &mut memo_table);

    max_value

}


fn main() {
    let file_2 = "knapsack_big.txt";
    let file_1 = "knapsack1.txt";

    let max_value_small_knap = run_knapsack(file_1);

    println!("Max value is {} for small knapsack", max_value_small_knap);
    

    let max_value_big_knap = run_knapsack(file_2);

    println!("Max value is {} for big knapsack", max_value_big_knap);

    




  






    //let file_big = File::open("knapsack_big.txt").unwrap();
    //let buf_file_big = BufReader::new(file_big);

    
}
