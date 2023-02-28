
pub fn tests() {
    let mut coll = AveragedCollection{
        list: vec![],
        average: 0.0,
    };
    println!("{:?}", coll);

    coll.average = 10.0;

    println!("{:?}", coll);
}


#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, val: i32) {
        self.list.push(val);
    }
}
