fn draw_triangle(height: usize, max_width: usize) -> impl Iterator<Item = String> {
    (0..height).map(move |i| {
        let stars = "*".repeat(2 * i + 1);
        let padding = (max_width - stars.len()) / 2;
        format!("{}{}", " ".repeat(padding), stars)
    })
}

fn draw_christmas_tree(triangles: usize) {
    let max_height = triangles;
    let max_width = 2 * (max_height + triangles - 1) + 1;

    for t in 1..=triangles {
        for line in draw_triangle(t, max_width) {
            println!("{}", line);
        }
    }
}

fn main() {
    let triangle_count = 5; // змінюй це число як хочеш
    draw_christmas_tree(triangle_count);
}
