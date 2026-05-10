use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn merge_sort_recursive(arr: &mut [u32], buffer: &mut [u32], inv_cnt: &mut u64){
    
    let len = arr.len();

    // base case
    if len <= 1{
        return;
    };

    let m = arr.len()/2;


    let (array_left, array_right) = arr.split_at_mut(m); // we need to use split at mut due to the fact that arr is a mutable reference

    let (buf_left, buf_right) = buffer.split_at_mut(m);


    merge_sort_recursive(array_left, buf_left, inv_cnt); // if len = 1, will just return a cloned version of itself. otherwise, sorts array_left in place

    merge_sort_recursive(array_right, buf_right, inv_cnt); // if len = 1, will just return a cloned version of itself. otherwise, sorts array_right in place


    // non-recursive merge process
    
    let len_left = array_left.len();

    let len_right = array_right.len();
    
    let mut index_left = 0;
    let mut index_right = 0;

    let mut index_buf = 0;

    
    while index_left < len_left && index_right < len_right{
    
    
        if array_left[index_left] > array_right[index_right]{
            buffer[index_buf] = array_right[index_right];
            let remaining_left_elements: u64 = (len_left - index_left) as u64;

            *inv_cnt += remaining_left_elements;
            
            index_right += 1;


            // add a step for counting cross inversions
        

    } else {
            buffer[index_buf] = array_left[index_left];
            index_left += 1;
    }

    index_buf += 1;
}

    // if there are remaining elements in the left array (while the right array is empty), append

    if index_left < len_left{
        buffer[index_buf..len].copy_from_slice(&array_left[index_left..]);
    }

    // Same for right side

    else if index_right < len_right{
        buffer[index_buf..len].copy_from_slice(&array_right[index_right..]);
    }

    arr.copy_from_slice(&buffer[0..len]);



}

fn count_inversion(arr: &[u32]) -> u64{
    let mut inversion_count: u64 = 0;

    let mut buffer: Vec<u32> = vec![0; arr.len()];

    let mut mut_arr = arr.to_vec();

    merge_sort_recursive(&mut mut_arr, &mut buffer, &mut inversion_count);

    inversion_count

}




fn main() {
    let file = File::open("IntegerArray.txt").expect("Failed to open file!");
    let buf_file = BufReader::new(file);

    let mut numbers: Vec<u32> = Vec::new();

    for line in buf_file.lines(){
        

        let line_trim = line.unwrap().trim().parse::<u32>().expect("Failed to read line");

        numbers.push(line_trim);
    }



    let inversion_count = count_inversion(&numbers);

    println!("{}", inversion_count);

 




    

}
