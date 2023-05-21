#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

#[derive(Debug)]
pub struct StringHolder<'a> {
    s: &'a str,
    t: &'a str,
}

fn two_string<'a>(s: &'a str, t: &'a str) -> StringHolder<'a> {
    StringHolder {
        t,
        s,
    }
}

pub fn lifetime_checker1() {
    let s = "Hello";
    let p = "world";

    let x =two_string(s,p);
    println!("{:?}",x)
}