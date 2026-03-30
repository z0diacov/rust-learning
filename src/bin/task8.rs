fn main() {
    let point = (12, 25);

    match point {
        (0, 0) => println!("Center"),
        (x, 0) => println!("On the X axis and x = {x}"),
        (0, y) => println!("On the Y axis and y = {y}"),
        (x, y) => println!("Not on the axis but on coords ({x}, {y})")
    };
}