#![allow(dead_code)]

use String;

pub fn tests() {
    traits_and_impl();

    traits_as_parameter();

    // return_trait_type();
}


// Define a trait named `Summary`
trait Summary {
    // trait function without default implementation
    fn get_title(&self) -> String;

    // trait function with default implementation
    fn get_name(&self) -> String {
        return String::from("default name");
    }
}

struct Blog {
    pub title: String,
    pub context: String,
}

// implement trait `Summary` for Blog
// the trait function `get_name()` not implemented here.
impl Summary for Blog {
    fn get_title(&self) -> String {
        return format!("Blog:{}", self.title);
    }
}


struct Twitter {
    pub username: String,
    pub context: String,
}

// implement trait `Summary` for Twitter.
impl Summary for Twitter {
    fn get_title(&self) -> String {
        return format!("Twitter:{}", self.username);
    }

    fn get_name(&self) -> String {
        return format!("{}", self.username);
    }
}



fn traits_and_impl() {
    println!("two structs implement the trait's function get_title()");
    {
        let blog = Blog {
            title: "blog-1".to_string(),
            context: "context-1".to_string()
        };
        let blog_title = blog.get_title();
        println!("blog title:{}", blog_title);  // blog-1

        let twitter = Twitter {
            username: "name-1".to_string(),
            context: "context-1".to_string(),
        };
        println!("twitter title:{}", twitter.get_title());  // name-1
    }

    println!("\nthe traits has a default function, two structs, and implemented \
    the function, another not");
    {
        let blog = Blog {
            title: "title-2".to_string(),
            context: "context-2".to_string(),
        };

        // struct Blog do not implement the traits function, get_name();
        // it will call the default implement of traits, and return 'default name'
        let blog_name = blog.get_name();
        println!("blog_name:{}", blog_name);  // default name

        let twitter = Twitter {
            username: "username-2".to_string(),
            context: "twitter-context-2".to_string(),
        };
        // Twitter implemented the traits function, get_name();
        // it will call the Twitter's function get_name().
        let twitter_name = twitter.get_name();
        println!("twitter_name:{}", twitter_name);  // username-2
    }
}


/*
 every struct that implemented Summary traits get_name() function
 can be passed to this function.
 */
fn get_name_trait(item: &impl Summary) {
    let item_name = item.get_name();
    println!("Summary item_name:{}", item_name);
}


/*
two parameters of two_trait_struct_parameter() can be different struct type.
e.g: (Blog, Twitter) / (Blog, Blog) / (Twitter, Twitter)
 */
fn two_trait_struct_parameter(item1: &impl Summary, item2: &impl Summary) {
    println!("{},{}", item1.get_name(), item2.get_name());
}


/*
the two parameters of two_trait_parameter_must_be_the_same() must be the same type.
e.g: (Blog, Blog) / (Twitter, Twitter)
 */
fn two_trait_parameter_must_be_the_same<T: Summary>(item1: &T, item2: &T) {
    println!("{},{}", item1.get_name(), item2.get_name());
}

fn traits_as_parameter() {
    println!("normal one trait parameter.");
    {
        let blog = Blog {
            title: "title-1".to_string(),
            context: "context-1".to_string(),
        };
        get_name_trait(&blog); // default name

        let twitter = Twitter {
            username: "username-2".to_string(),
            context: "context-2".to_string(),
        };
        get_name_trait(&twitter);  // username-2s
    }

    println!("\ntwo different struct of traits");
    {
        let blog = Blog {
            title: "title-1".to_string(),
            context: "context-1".to_string(),
        };

        let twitter = Twitter {
            username: "username-2".to_string(),
            context: "context-2".to_string(),
        };

        // default name, username-1
        two_trait_struct_parameter(&blog, &twitter);

        // error: expected `&Blog`, found `&Twitter`
        // two_trait_parameter_must_be_the_same(&blog, &twitter);

        let blog_2 = Blog {
            title: "title-3".to_string(),
            context: "context-3".to_string(),
        };
        // default name, default name
        two_trait_parameter_must_be_the_same(&blog, &blog_2);
    }
}

/*
return traits type
 */
fn return_trait_type() {
    let s1 = return_summary_trait_type(true);
    let s2 = return_summary_trait_type(false);
    println!("{},{}", s1.get_title(), s2.get_title());  // title-1, title-2
}

/*
if the traits type return function has multiple return expression,
all return expression must has the same concrete type.
This is the same as c++11's type deduction.
 */
fn return_summary_trait_type(t: bool) -> impl Summary {
    return if t {
        Blog {
            title: "title-1".to_string(),
            context: "context-1".to_string(),
        }
    } else {
        Blog {
            title: "title-2".to_string(),
            context: "context-2".to_string(),
        }

        // error: `if` and `else` have incompatible types.
        // Twitter {
        //     username: "username".to_string(),
        //     context: "context-twitter".to_string(),
        // }
    }
}

