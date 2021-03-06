// https://burtleburtle.net/bob/rand/smallprng.html

use std::cell::RefCell;
use std::num::Wrapping;

thread_local! { static RAND: RefCell<Rand> = RefCell::new(Rand::new_from_time()); }

#[derive(Copy, Clone)]
pub struct Rand {
    a: Wrapping<u32>,
    b: Wrapping<u32>,
    c: Wrapping<u32>,
    d: Wrapping<u32>,
}

impl Rand {
    pub fn new(seed: u32) -> Self {
        #[allow(clippy::unreadable_literal)]
        let mut rand = Rand {
            a: Wrapping(0xf1ea5eed),
            d: Wrapping(seed),
            c: Wrapping(seed),
            b: Wrapping(seed),
        };
        for _ in 0..20 {
            rand.next();
        }
        rand
    }

    pub fn new_from_time() -> Self {
        use std::time::SystemTime;

        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Rand::new(seed as _)
    }

    pub fn next(&mut self) -> u32 {
        let rot = |x, k| (x << k) | (x >> (32 - k));
        let e = self.a - rot(self.b, 27);
        self.a = self.b ^ rot(self.c, 17);
        self.b = self.c + self.d;
        self.c = self.d + e;
        self.d = e + self.a;
        self.d.0
    }

    pub fn next_f32(&mut self) -> f32 {
        self.next() as f32 / u32::MAX as f32
    }

    pub fn next_between_f32(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }
}

pub fn rand() -> f32 {
    RAND.with(|r| r.borrow_mut().next_f32())
}

pub fn rand_between(min: f32, max: f32) -> f32 {
    RAND.with(|r| r.borrow_mut().next_between_f32(min, max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_rand() {
        let seed = 42;
        let mut rand = Rand::new(seed);

        let mut v = 0;
        for _ in 0..100 {
            v = rand.next();
        }

        assert_eq!(v, 3863832633);
    }

    #[test]
    fn tests_rand_next_f32() {
        let seed = 42;
        let mut rand = Rand::new(seed);

        let mut v = 0.0;
        for _ in 0..100 {
            v = rand.next_f32();
        }

        assert_eq!(v, 0.8996186);
    }

    #[test]
    fn tests_rand_time_seed() {
        let mut rand = Rand::new_from_time();
        assert_ne!(rand.next(), 0);
    }
}
