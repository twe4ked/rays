// https://burtleburtle.net/bob/rand/smallprng.html

#[derive(Copy, Clone)]
pub struct Rand {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl Rand {
    pub fn new(seed: u32) -> Self {
        let mut rand = Rand {
            a: 0xf1ea5eed,
            d: seed,
            c: seed,
            b: seed,
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
        let e = self.a.wrapping_sub(rot(self.b, 27));
        self.a = self.b ^ rot(self.c, 17);
        self.b = self.c.wrapping_add(self.d);
        self.c = self.d.wrapping_add(e);
        self.d = e.wrapping_add(self.a);
        self.d
    }
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
    fn tests_rand_time_seed() {
        let mut rand = Rand::new_from_time();
        assert_ne!(rand.next(), 0);
    }
}
