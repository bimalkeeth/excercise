#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::cell::RefCell;

use std::rc::Rc;

// #[derive(Debug)]
// pub struct WithLife<'a> {
//     s: &'a String,
// }
#[derive(Debug)]
pub struct NoLife{
    s:Rc<RefCell<String>>
}


fn make_no_life(fname:&str)->Result<(NoLife,NoLife),std::io::Error>{
    let s =std::fs::read_to_string(fname)?;
    let r =Rc::new(RefCell::new(s));
    Ok((NoLife{s:r.clone()},NoLife{s:r}))
}

// fn make_with_life<'a>(fname: &str) -> Result<(WithLife<'a>, WithLife<'a>), std::io::Error> {
//     let s = std::fs::read_to_string(fname)?;
//
//     Ok((WithLife{s:&s},WithLife{s:&s}))
// }