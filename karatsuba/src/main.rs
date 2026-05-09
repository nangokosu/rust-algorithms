
fn add(a: &[u8], b: &[u8]) -> Vec<u8>{
    let mut result = Vec::new();
    let mut carry = 0;
    let max_len = std::cmp::max(a.len(), b.len());
    for i in 0..max_len{
        let digit_a = a.get(i).cloned().unwrap_or(0);
        let digit_b = b.get(i).cloned().unwrap_or(0); // the number with the smaller size can be considered to be zero-padded to the right

        let sum = digit_a + digit_b + carry;
        result.push(sum % 10);
        carry = sum/10; // technically, the max carry possible is just 1
    }

    if carry > 0{
        result.push(carry); // if at the very end (where the high digit is), there is still carry (max = 1), 
        // we append it to the array (making it the highest digit right now)
    }
    result //still in reverse order
}

fn subtract(a: &[u8], b: &[u8]) -> Vec<u8>{
    let mut result = Vec::new();
    let mut borrow = 0;

    for i in 0..a.len(){
        let digit_a = a.get(i).cloned().unwrap_or(0) as i32;// converting because a < b
        let digit_b = b.get(i).cloned().unwrap_or(0) as i32;

        let mut diff = digit_a - digit_b - borrow; 

        if diff < 0 {
            diff += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }

        result.push(diff as u8); //this has lower bound 0, we do not assume negative
    }

    while result.len() > 1 && result.last() == Some(&0) {
        result.pop();
    }
    result // still in reverse order

}


fn shift_left(digits: &[u8], k: usize) -> Vec<u8> {
    if digits.is_empty() || (digits.len() == 1 && digits[0] == 0) {
        return vec![0];
    }
    // Since the vector is reversed, multiplying by 10^k means 
    // prepending k zeros to the front of the vector.
    let mut shifted = vec![0; k];
    shifted.extend_from_slice(digits);
    shifted
}



fn karatsuba(x: &[u8], y: &[u8]) -> Vec<u8>{
    // Splitting into requisite numbers:

    let n = std::cmp::max(x.len() , y.len());

    if n <= 1 {
        let val_x = x.first().cloned().unwrap_or(0); // .first() returns an Option, whereas x[0] would have caused a panic
        let val_y = y.first().cloned().unwrap_or(0); // note that x or y can be null 
        // based on the assignment below (in the case where x & y have uneven sizes)
        let prod = val_x * val_y;

        if prod >= 10{
            return vec![prod % 10, prod /10] // still in reverse order, so first vector element is last digit
        } else {
            return vec![prod]
        }

    }

    let m = n/2;

    let x_low = if x.len() > m {&x[0..m]} else {x}; // again, reverse logic, so low is at the start, high is at the end
    let x_high = if x.len() > m {&x[m..]} else {&[]};

    let y_low = if y.len()> m {&y[0..m]} else {y};
    let y_high = if y.len()> m {&y[m..]} else {&[]};

    let p1 = karatsuba(x_high, y_high); //ac
    let p2 = karatsuba(x_low, y_low); //bd

    let sum_x = add(x_high, x_low);
    let sum_y = add(y_high, y_low);

    let p3 = karatsuba(&sum_x, &sum_y);


    let p3_minus_p1 = subtract(&p3, &p1);

    let middle_term = subtract(&p3_minus_p1, &p2);

    let term1 = shift_left(&p1, 2 * m); // largest digits

    let term2 = shift_left(&middle_term, m); //middle digits

    let partial_sum = add(&term1, &term2);


    add(&partial_sum, &p2)




}




fn main() {
    let x: String = "3141592653589793238462643383279502884197169399375105820974944592".to_string();
    let y: String = "2718281828459045235360287471352662497757247093699959574966967627".to_string();

    // turning the long numbers into Vectors of u8, but in reverse

    let x_vec: Vec<u8> = x.chars().map(|s: char| s.to_digit(10).unwrap() as u8).rev().collect(); 
    let y_vec: Vec<u8> = y.chars().map(|s: char| s.to_digit(10).unwrap() as u8).rev().collect();

    let result_reversed = karatsuba(&x_vec, &y_vec);

    let final_product: String = result_reversed.iter().rev().map(|d| d.to_string()).collect();

    println!("Multiplication Result:\n{}", final_product);




}
