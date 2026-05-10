use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;




fn stanford_partition(slice: &mut [u32]) -> usize {
    let n = slice.len();
    let mut i = 1;

    for j in 1..n{
        if slice[j] < slice[0]{ // note that we assume the pivot is at 0 index
            slice.swap(i,j); // swaps element of vector in place
            i += 1
        }
    }

    slice.swap(0, i-1); //swaps element of vector in place
    i-1  // returns the final index where the pivot lands
}


pub fn quicksort_count<F>(slice: &mut [u32], mut choose_pivot: F) -> u64
where F: FnMut(&mut [u32]) + Copy
// also sorts the array in place
{
   

    let n = slice.len();

    if n <= 1 {
        return 0
    }

    let mut comparisons: u64 = (n - 1) as u64;

    choose_pivot(slice); //determines the pivot and moves it to index 0 in-place

    let pivot_idx = stanford_partition(slice);

    let (left, right_with_pivot) = slice.split_at_mut(pivot_idx); // two mutable references

    let right = &mut right_with_pivot[1..]; // excluding the pivot

    comparisons += quicksort_count(left, choose_pivot);

    comparisons += quicksort_count(right, choose_pivot);

    comparisons



}










fn main() {
    let file = File::open("IntegerArrayQuickSort.txt").expect("Failed to open file!");
    let buf_file = BufReader::new(file);

    let mut numbers: Vec<u32> = Vec::new();

    for line in buf_file.lines(){
        

        let line_trim = line.unwrap().trim().parse::<u32>().expect("Failed to read line");

        numbers.push(line_trim);
    }

    let mut numbers_1: Vec<u32> = numbers.clone();

    // always using the first index as pivot

    let count_comparisons_1 = quicksort_count(&mut numbers_1, |_slice|{});

    println!("Number of comparisons for choosing first pivot: {}", count_comparisons_1);


    let mut numbers_2: Vec<u32> = numbers.clone();

    let count_comparisons_2 = quicksort_count(&mut numbers_2, |slice|{
        let last_idx = slice.len() - 1;
        slice.swap(0, last_idx); // the pivot is the last element, and we force it to become the 0th index
    });


    println!("Number of comparisons for choosing last pivot: {}", count_comparisons_2);

    let mut numbers_3: Vec<u32> = numbers.clone();
    let count_comparisons_3 = quicksort_count(&mut numbers_3, |slice|{
        let n = slice.len();
        if n > 2{
            let first = 0;
            let last = n -1;
            let middle = (n-1)/2;


            let a = &slice[first];
            let b = &slice[middle];
            let c = &slice[last];


            let median_idx = if (a >= b && a <= c) || (a <= b && a >= c) {
            first} 
            else if (b >= a && b <= c) || (b <= a && b >= c) {
            middle} 
            else {
            last};

        slice.swap(0, median_idx); // determines median, and then swaps this to being the 0th index
        }
    });

    println!("Number of comparisons for choosing median-of-three: {}", count_comparisons_3);

    




    

}
