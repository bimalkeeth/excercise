#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]


use rayon::join;

fn on_each<T, F>(v: &mut [T], f: F) where F:Fn(&mut T)+Send+Copy +Sync, T:Send{
    match v.len() {
        0 => (),
        n if n < 4 => {
            for i in v {
                f(i)
            }
        }
        n => {
            let (v1, v2) = v.split_at_mut(n / 2);
            join(||on_each(v1,f),||on_each(v2,f));
        }
    }
}