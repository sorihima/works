use rand::Rng;

fn count_permutation(shipments: &Vec<u32>) -> isize {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();

    if total as usize % n != 0 {
        return -1; 
    }

    let avg = total / n as u32;
    let mut moves = 0;
    let mut imbalance = 0;

    for &weight in shipments.iter() {
        imbalance += weight as i32 - avg as i32;
        moves += imbalance.abs() as isize;
    }

    moves
}


fn count_permutation_result(shipments: &Vec<u32>) -> Result<usize, &'static str> {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();

    if total as usize % n != 0 {
        return Err("Impossible to balance cargo equally");
    }

    let avg = total / n as u32;
    let mut moves = 0;
    let mut imbalance = 0;

    for &weight in shipments.iter() {
        imbalance += weight as i32 - avg as i32;
        moves += imbalance.abs() as usize;
    }

    Ok(moves)
}


fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(1..=10);
    let mut result = vec![avg; n];

    for _ in 0..n / 2 {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        if result[i] > 1 {
            result[i] -= 1;
            result[j] += 1;
        }
    }

    result
}

fn main() {
    let example1 = vec![1, 1, 1, 1, 6];
    println!("Example 1: {:?} -> Moves: {}", example1, count_permutation(&example1));

    let example2 = vec![8, 2, 2, 4, 4];
    println!("Example 2: {:?} -> Moves: {}", example2, count_permutation(&example2));

    let example3 = vec![9, 3, 7, 2, 9];
    println!("Example 3: {:?} -> Moves: {}", example3, count_permutation(&example3));

    let invalid = vec![1, 2, 3];
    println!("Invalid Example: {:?} -> Moves: {}", invalid, count_permutation(&invalid));

   
    let generated = gen_shipments(6);
    println!("Generated Valid Shipments: {:?} -> Moves: {}", generated, count_permutation(&generated));

    
    match count_permutation_result(&example1) {
        Ok(m) => println!("(Result version) Moves: {}", m),
        Err(e) => println!("(Result version) Error: {}", e),
    }
}
