#![allow(dead_code)]

pub fn struct_method_tests() {
    cal_rectangle_area();

    associated_function();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn cal_rectangle_area() {
    let mut rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    let area1 = rect1.area();
    println!("area1 of {:?} is:{}", rect1, area1);  // 200

    rect1.set_width(20);
    let area2 = rect1.area();
    println!("area2 of {:?} is:{}", rect1, area2);  // 400


}

fn associated_function() {
    let square1 = Rectangle::square(10);
    let area1 = square1.area();
    println!("area of {:?} is {}", square1, area1);  // 100
}