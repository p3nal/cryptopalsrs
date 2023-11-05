// y = y ^ (y >> self.constants.u);
// y = y ^ ((y << self.constants.s) & self.constants.b);
// y = y ^ ((y << self.constants.t) & self.constants.c);
// y = y ^ (y >> self.constants.l);
//
//  orig = 10100101
//             101101
//     y = 101100
//
//
//   101000000
//   00000101000000
//   101001010
//
//
//   101000000
// ^ 000000000
// & 111110000
//   10100
//
use crate::s3::s3ch21::MT19937;

pub enum ShiftDirection {
    Left,
    Right,
}

pub trait RevShift {
    fn rev_sh(&self, shift: u32, and: u32, shift_direction: ShiftDirection) -> u32;
}

impl RevShift for u32 {
    fn rev_sh(&self, shift: u32, and: u32, shift_direction: ShiftDirection) -> u32 {
        let mut orig = 0 as u32;
        (0..32 as u32)
            .into_iter()
            .step_by(shift as usize)
            .for_each(|i| {
                match shift_direction {
                    ShiftDirection::Left => {
                        let mask = match 0xffffffff_u32.checked_shl(i + shift) {
                            Some(m) => !m,
                            None => 0xffffffff,
                        };
                        orig |= (self ^ (orig << shift) & and) & mask;
                    }
                    ShiftDirection::Right => {
                        let mask = match 0xffffffff_u32.checked_shr(i + shift) {
                            Some(m) => !m,
                            None => 0xffffffff,
                        };
                        orig |= (self ^ (orig >> shift) & and) & mask;
                    }
                };
            });
        orig
    }
}

pub fn clone_mt19937(outputs: Vec<u32>) -> Vec<u32> {
    let (b, c, s, t, u, l, d) = (
        0x9D2C5680u32,
        0xEFC60000u32,
        7u32,
        15u32,
        11u32,
        18u32,
        0xffffffffu32,
    );
    outputs
        .iter()
        .map(|&y| {
            y.rev_sh(l, d, ShiftDirection::Right)
                .rev_sh(t, c, ShiftDirection::Left)
                .rev_sh(s, b, ShiftDirection::Left)
                .rev_sh(u, d, ShiftDirection::Right)
        })
        .collect::<Vec<u32>>()
}

pub fn splice_mt19937() {
    let mut mt = MT19937::new(1337); // the secret seed.....
    let mut cloned_mt = MT19937::new(0xdeadbeef); // random seed here we dont care
    let mut tapped_output: Vec<u32> = Vec::new();
    for _ in 0..624 {
        // tapping...
        tapped_output.push(mt.extract_number());
    }
    // cloning...
    let cloned_state = clone_mt19937(tapped_output);
    // splicing...
    cloned_mt.mt = cloned_state;
    // et voila
    for _ in 0..20 {
        println!("orig = {}, cloned = {}", mt.extract_number(), cloned_mt.extract_number());
    }
}


