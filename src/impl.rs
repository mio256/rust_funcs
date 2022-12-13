#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 40,
        height: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle1.area()
    );

    dbg!(rectangle1);
}
