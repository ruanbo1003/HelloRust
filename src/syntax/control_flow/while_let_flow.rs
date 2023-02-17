
pub fn tests() {
    let mut cond = Some(0);

    while let Some(i) = cond {
        if i > 10 {
            println!("Quit");
            cond = None;
        } else {
            cond = Some(i + 1);
            println!("{:?}", cond);
        }
    }
}