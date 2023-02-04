#![allow(dead_code)]

pub fn tests() {
    one_match_branch();
}

fn one_match_branch() {
    {
        let config_max = Some(3u8);
        match config_max {
            Some(val) => println!("Some(val):{}", val),
            _ => (),
        }
    }

    {
        let config_max = Some(3u8);
        if let Some(val) = config_max {
            println!("Some val:{}", val)
        }
    }

    {
        let config_max: Option<u8> = None;
        if let Some(val) = config_max {
            println!("some val:{}", val);
        } else {
            println!("val is None");  // val is None
        }
    }
}

