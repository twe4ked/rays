// https://burtleburtle.net/bob/rand/smallprng.html

#[derive(Copy, Clone)]
pub struct Rand {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

impl Rand {
    pub fn new(seed: u64) -> Self {
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
        Rand::new(seed)
    }

    pub fn next(&mut self) -> u64 {
        let e = self.a.wrapping_sub(self.b << 27 | self.b >> 32 - 27);
        self.a = self.b ^ (self.c << 17 | self.c >> 32 - 17);
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

        assert_eq!(v, 6895266172536598682);
    }

    #[test]
    fn tests_rand_time_seed() {
        let mut rand = Rand::new_from_time();
        assert_ne!(rand.next(), 0);
    }
}
