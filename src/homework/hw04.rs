const WIDTH: usize = 7; 
const HEIGHT: usize = 4; 

fn main() {
    let mut output = String::new();

    for i in 0..HEIGHT {
        for _ in 0..(WIDTH / 2 - i) {
            output.push(' ');
        }
        output.push('/');
        for _ in 0..(i * 2) {
            output.push(' ');
        }
        output.push('\\');
        output.push('\n');
    }

    output.push_str(&"-".repeat(WIDTH));
    output.push('\n');

    for i in (0..HEIGHT).rev() {
        for _ in 0..(WIDTH / 2 - i) {
            output.push(' ');
        }
        output.push('\\');
        for _ in 0..(i * 2) {
            output.push(' ');
        }
        output.push('/');
        output.push('\n');
    }

    print!("{}", output);
}
