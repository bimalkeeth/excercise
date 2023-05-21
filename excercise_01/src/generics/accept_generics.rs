#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::ops::AddAssign;

#[derive(PartialEq, Debug)]
pub struct USD(i32);

#[derive(PartialEq, Debug)]
pub struct GBP(i32);

#[derive(PartialEq, Debug)]
pub struct CAD(i32);


pub trait ToUsdVal<F> {
    fn to_uv(&self, g: F) -> f32;
}

pub trait FromUsdVal<F> {
    fn from_uv(&self, g: f32) -> F;
}

pub struct Ex {
    cad: f32,
    gbp: f32,
}

impl ToUsdVal<GBP> for Ex {
    fn to_uv(&self, g: GBP) -> f32 {
        (g.0 as f32) * self.gbp
    }
}

impl FromUsdVal<CAD> for Ex {
    fn from_uv(&self, g: f32) -> CAD {
        CAD((g / self.cad) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let g = GBP(200);
        let ex = Ex { cad: 0.7, gbp: 1.3 };
        let c = ex.from_uv(ex.to_uv(g));
        assert_eq!(c, CAD(371))
    }
}

//---------------Iterators generics ---------------------

pub struct Stepper<T> {
    curr: T,
    step: T,
    stop: T,
}

impl<T> Stepper<T> {
    pub fn new(start: T, stop: T, step: T) -> Self {
        Stepper {
            stop,
            step,
            curr: start,
        }
    }
}

impl<T> Iterator for Stepper<T> where T: AddAssign + Copy + PartialOrd {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.curr >= self.stop {
            return None;
        }

        let res = self.curr;
        self.curr += self.step;

        Some(res)
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn it_works() {
        let mut c = 0;
        for n in Stepper::new(2, 10, 2) {
            c += n;
        }
    }
}


//-----------------------enum definition --------------------------------

#[derive(Default,Clone)]
struct Player {
    name: String,
    iq: u8,
    friends: u8,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            ..Default::default()
        }
    }
    pub fn get_friends(&self) -> u8 {
        self.friends
    }

    pub fn set_friends(&mut self, friends: u8) {
        self.friends = friends
    }
}



pub fn init_player() {
    let pl = Player::new("samson");
    let mut s =pl;
    s.set_friends(20)
}