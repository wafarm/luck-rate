const MASK: i64 = (1i64 << 48) - 1;
const MULTIPLIER: i64 = 0x5DEECE66Di64;
const ADDEND: i64 = 0xBi64;

pub struct Random {
    seed: i64,
}

impl Random {
    pub fn new(seed: i64) -> Self {
        let seed = (seed ^ MULTIPLIER) & MASK;

        Self { seed }
    }

    fn next(&mut self, bits: i32) -> i32 {
        self.seed = (self.seed * MULTIPLIER + ADDEND) & ((1i64 << 48) - 1);
        ((self.seed as u64) >> (48 - bits)) as i32
    }

    pub fn next_i32(&mut self) -> i32 {
        self.next(32)
    }

    #[allow(unused_assignments)]
    pub fn bounded_next_i32(&mut self, origin: i32, bound: i32) -> i32 {
        let mut r = self.next_i32();
        if origin < bound {
            let n = bound - origin;
            let m = n - 1;
            if (n & m) == 0 {
                r = (r & m) + origin;
            } else if n > 0 {
                let mut u = ((r as u32) >> 1) as i32;
                loop {
                    r = u % n;
                    if u + m - r < 0 {
                        r += origin;
                        u = ((self.next_i32() as u32) >> 1) as i32;
                    } else {
                        break;
                    }
                }
            } else {
                while r < origin || r >= bound {
                    r = self.next_i32();
                }
            }
        }
        r
    }
}
