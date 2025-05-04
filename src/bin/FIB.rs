fn main() {
    let output: u128 = fibonacci(35,4);
    println!("{}",output);

}

// Had to use u128 as u32 could not hold the required integer sizes
fn fibonacci(n:u128,k:u128) -> u128 {
    if n==0 {
        0
    } else if n == 1 || n==2 {
        1
    } else {
        // The rabbits that from the last generation are present, while those from two generations ago have all produced k offspring
        fibonacci(n-1,k)+(fibonacci(n-2,k)*k)
    }

}
