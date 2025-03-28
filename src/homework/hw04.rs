const SIZE: usize = 5; 

fn main() {
    let mut output = String::new();
  
    for i in 0..SIZE {
        for _ in 0..(SIZE - i - 1) {
            output.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            output.push('*');
        }
        output.push('\n');
    }

    for _ in 0..(2 * SIZE + 1) {
        output.push('*');
    }
    output.push('\n');

    for i in (0..SIZE).rev() {
        for _ in 0..(SIZE - i - 1) {
            output.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            output.push('*');
        }
        output.push('\n');
    }

    print!("{}", output);
}
