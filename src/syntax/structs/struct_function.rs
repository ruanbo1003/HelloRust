
pub fn tests() {
    // immutable struct instance, call the function with `&self` parameter.
    {
        let st = ST::build_st();
        println!("{:?}", st);  // ST { data: 1, name: "abc" }

        st.display();  // ST:1, abc
    }

    // mutable struct instance, call the function with `&mut self` parameter.
    {
        let mut st = ST::build_st();
        st.set_data(100);

        st.display(); // ST:100, abc
    }

}

#[derive(Debug)]
struct ST {
    data: i32,
    name: String,
}

// member function of struct
impl ST {
    fn display(&self) {
        println!("ST:{}, {}", self.data, self.name)
    }

    fn set_data(&mut self, data: i32) {
        self.data = data
    }
}

// associate function
// one struct can have multiple `impl`.
impl ST {
    fn build_st() -> ST {
        ST {
            data: 1,
            name: "abc".to_string()
        }
    }
}
