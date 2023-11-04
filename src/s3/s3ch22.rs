// mt19937 works fine...
//
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rand::Rng;

use super::s3ch21::MT19937;

pub fn go_get_coffee_while_this_runs() -> u32 {
    let mut rng = rand::thread_rng();
    let random_period = rng.gen_range(40..=1000);
    let random_num_of_millis = Duration::from_secs(random_period);
    sleep(random_num_of_millis);
    let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs() as u32,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    println!("fyi orig seed = {seed}");
    let mut mt = MT19937::new(seed);
    let random_period = rng.gen_range(40..=1000);
    let random_num_of_millis = Duration::from_secs(random_period);
    sleep(random_num_of_millis);
    let rand = mt.extract_number();
    rand
}

pub fn crack_mt19937_seed(rng: u32) -> u32 {
    let mut seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs() as u32,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    loop {
        let mut mt = MT19937::new(seed);
        let next = mt.extract_number();
        if rng == next {
            break
        }
        seed -= 1;
    }
    seed
}
