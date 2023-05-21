#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn thread_tester() {
    with_arc()
}

fn with_arc() {
    let m = Arc::new(Mutex::new(String::from("moving")));
    let m2 = m.clone();

    std::thread::spawn(move || {
        println!("this is the new thread");
        let mut s = m2.lock().unwrap();

        s.push_str(" on the new thread ");
        println!("m = {}", s)
    });

    std::thread::sleep(Duration::from_millis(1000));
    println!("this is the initial thread");
    let s = m.lock().unwrap();
    println!("now m = {}", s)
}


fn with_channels() {
    let (chs, chr) =
        std::sync::mpsc::channel::<Box<dyn Fn(&mut String) + Send>>();

    let (dones,doner) =std::sync::mpsc::channel::<()>();

    std::thread::spawn(move|| {
        let mut hidden = String::new();
        loop {
            match chr.recv() {
                Ok(f)=>{
                    f(&mut hidden);
                    println!("hidden {}", hidden);
                },
                Err(_)=>{
                    println!("Done");
                    dones.send(()).expect("error in sending");

                    return;
                }
            }

        }
    });

    chs.send(Box::new(|s: &mut String| {
        s.push_str("hello");
    })).expect("error happened");

    drop(chs);
    //drop(chr);

    doner.recv().ok();
}