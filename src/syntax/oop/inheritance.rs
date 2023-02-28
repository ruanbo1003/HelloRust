
pub fn tests() {
    screen_gui();
}

trait Draw {
    fn draw(&self);
}

struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for each in self.components.iter() {
            each.draw();
        }
    }
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}
impl Draw for Button {
    fn draw(&self) {
        // draw button
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw SelectBox
    }
}

fn screen_gui() {
    let screen = Screen {
        components: vec![
            Box::new(
                Button {
                    width: 10,
                    height: 10,
                    label: String::from("OK")
                }
            ),
            Box::new(SelectBox {
                width: 10,
                height: 20,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                ]
            })
        ]
    };

    screen.run();
}