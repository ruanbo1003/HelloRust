
enum WebEvent {
    PageLoad,
    PageClose,
    KeyPress(char),
    Click{x:i32, y:i32},
}

type WE = WebEvent;

fn web_event_1(we: &WE) {
    match we {
        WE::PageLoad => println!("page load"),
        WE::PageClose=> {
            println!("page close");
        },
        WE::KeyPress(c) => {
            println!("key {} press", c);
        },
        WE::Click{x, y} => {
            println!("click ({}, {})", x, y);
        }
    }
}

fn web_event_2(we: &WE) {
    match we {
        _WebEvent=> println!("web event"),
    }
}

fn web_event_3(we: &WE) {
    match we {
        WebEvent::PageLoad => {
            println!("page load");
        }
        _ => {
            println!("not page load");
        }
    }
}

pub fn tests() {
    let we = WE::PageLoad;
    web_event_1(&we);
    web_event_2(&we);

    let we2 = WE::Click {x: 10, y: 11};
    web_event_3(&we2);
}