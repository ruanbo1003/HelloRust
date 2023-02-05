#![allow(dead_code)]

use std::fmt::format;
use std::os::unix::raw::time_t;
use String;


struct Blog {
    pub title: String,
    pub context: String,
}

impl Summary for Blog {
    fn get_title(&self) -> String {
        return format!("Blog:{}", self.title);
    }
}


struct Twitter {
    pub username: String,
    pub context: String,
}

impl Summary for Twitter {
    fn get_title(&self) -> String {
        return format!("Twitter:{}", self.username);
    }

    fn get_name(&self) -> String {
        return format!("{}", self.username);
    }
}

trait Summary {
    fn get_title(&self) -> String;

    fn  get_name(&self) -> String {
        return String::from("default name");
    }
}

pub fn tests() {
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
