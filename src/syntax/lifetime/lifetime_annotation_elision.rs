
pub fn tests() {
    // first world
    {
        let s1 = "Hello World";
        let first = first_word_test(s1);
        println!("{:?}", first);  // Hello
    }
}

/*
the function `first_world_test` has one reference parameter, and one reference return value,
and we do not add the lifetime-annotation. It works fine, why?
 1. the function `first_world_test` is equal to `first_world_test_2`.
 */
fn first_word_test(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_world_test_2<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
