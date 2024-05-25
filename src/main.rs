use triangle_library::Triangle;

#[derive(Debug)]
enum Figure {
    Triangle(Triangle),
}

fn main() {
    let mut buffer: String = String::new();
    let mut sides: Vec<u32> = Vec::new();

    println!("*** Please enter each side of the triangle ***\n");

    for i in 0..=2 {
        println!("-> Enter side {}: ", i + 1);
        std::io::stdin().read_line(&mut buffer).unwrap();
        println!("Side {}: {}", i + 1, buffer);
        sides.push(buffer.trim().parse::<u32>().expect("Parsable"));
        buffer.clear();
    }

    let t = Figure::Triangle(Triangle::new(sides[0], sides[1], sides[2]));

    match t {
        Figure::Triangle(ref t) => {
            println!("{:?}", t);
            println!("Greater side: {}", Triangle::get_greater_side(&t));
            println!("Triangle type: {}", Triangle::get_triangle_type(&t));
        }
    }
}
