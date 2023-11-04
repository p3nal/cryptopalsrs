// impelmenting mt19937
//
// mainly stolen from wikipedia pseudocode
//

pub struct Constants {
    /*
     * w: word size (in number of bits)
     * n: degree of recurrence
     * m: middle word, an offset used in the recurrence relation defining the series x, 1 ≤ m < n
     * r: separation point of one word, or the number of bits of the lower bitmask, 0 ≤ r ≤ w − 1
     * a: coefficients of the rational normal form twist matrix
     * b, c: TGFSR(R) tempering bitmasks
     * s, t: TGFSR(R) tempering bit shifts
     * u, d, l: additional Mersenne Twister tempering bit shifts/masks
     */
    // w: u32,
    n: u32,
    m: u32,
    r: u32,
    a: u32,
    b: u32,
    c: u32,
    s: u32,
    t: u32,
    u: u32,
    // d: u32,
    l: u32,
    f: u32,
}

pub struct MT19937 {
    constants: Constants,
    mt: Vec<u32>,
    index: u32,
    lower_mask: u32,
    upper_mask: u32,
}

impl MT19937 {
    #[allow(unused)]
    pub fn new(seed: u32) -> MT19937 {
        let constants = Constants {
            n: 624,
            m: 397,
            r: 31,
            a: 0x9908b0df,
            b: 0x9D2C5680,
            c: 0xEFC60000,
            s: 7,
            t: 15,
            u: 11,
            l: 18,
            f: 1812433253,
        };
        let n = constants.n;
        let r = constants.r;
        let lower_mask = (1 << 31) - 1;
        let upper_mask = (1 << 31);
        let mut mt = vec![0; n as usize];
        mt[0] = seed;
        for i in 1_u32..constants.n {
            // supposing w is whatever type self.mt elements are... so we're not bothering with taking
            // lowest bits as its just casted...
            mt[i as usize] = constants
                .f
                .wrapping_mul((mt[(i - 1) as usize]) ^ (mt[(i - 1) as usize] >> 30))
                .wrapping_add(i);
        }
        MT19937 {
            constants,
            mt,
            index: n,
            lower_mask,
            upper_mask,
        }
    }

    // Generate the next n values from the series x_i
    fn twist(&mut self) {
        for i in 0..self.constants.n {
            let x: u32 = (self.mt[i as usize] & self.upper_mask)
                | (self.mt[((i + 1) % self.constants.n) as usize] & self.lower_mask);
            let mut x_a: u32 = x >> 1;
            if (x % 2) != 0 {
                // lowest bit of x is 1
                x_a = x_a ^ self.constants.a;
            }
            self.mt[i as usize] =
                self.mt[((i + self.constants.m) % self.constants.n) as usize] ^ x_a;
        }
        self.index = 0;
    }

    pub fn extract_number(&mut self) -> u32 {
        if self.index >= self.constants.n {
            self.twist();
        }

        let mut y: u32 = self.mt[self.index as usize];
        y = y ^ (y >> self.constants.u);
        y = y ^ ((y << self.constants.s) & self.constants.b);
        y = y ^ ((y << self.constants.t) & self.constants.c);
        y = y ^ (y >> self.constants.l);

        self.index += 1;
        y
    }
}
