//! bandit algorithm 
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::{thread_rng, Rng};
use statrs::distribution::Binomial;

// Arm struct
struct Arm {
    _p: f64,
    success: i32,
    fail: i32,
    binomial: Binomial,
}

impl Arm {
    pub fn new(p: f64) -> Self {
        Arm {
            _p: p,
            success: 0,
            fail: 0,
            binomial: Binomial::new(p, 1).unwrap(),
        }
    }
    /// trial Arm.
    pub fn play(&mut self) -> i32 {
        let mut rng = rand::rngs::StdRng::from_entropy();
        let result = self.binomial.sample::<StdRng>(&mut rng) as i32;
        if result == 1 {
            self.success += 1;
        } else {
            self.fail += 1;
        }
        result
    }

    /// calc success
    pub fn calc_success(&self) -> f64 {
        if self.success + self.fail == 0 {
            0.0
        } else {
            (self.success as f64) / (self.success + self.fail) as f64
        }
    }
}

/// ramdom
///
/// # Parameters
/// * `arms` - Vector of Arm
/// * `count` - count of get reward
fn ramdom_greedy(arms: &mut Vec<Arm>, count: i32) -> i32 {
    let mut reward = 0;

    for _i in 0..count {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..arms.len());
        reward += arms[index].play();
    }
    reward
}

/// search & utilization
///
/// # Parameters
/// * `arms` - Vector of Arm
/// * `count` - count of get reward
/// * `epsilon` - search rate parameter
fn epsilon_greedy(arms: &mut Vec<Arm>, count: i32, epsilon: f64) -> i32 {
    let mut reward = 0;

    for _i in 0..count {
        //epsiron(〜1.0)の確率で表がでるコインをふる
        let b = Binomial::new(epsilon, 1).unwrap();
        let mut rng = rand::rngs::StdRng::from_entropy();
        let index: usize;

        if b.sample(&mut rng) as i32 == 1 {
            // search: select random arm
            let mut rng = thread_rng();
            index = rng.gen_range(0..arms.len());
        } else {
            // utilization : Choose the arm with the highest probability of success ever
            index = select_highest_arm_index(&arms);
        }
        reward += arms[index].play();
    }
    reward
}

/// select highest Arm Index
///
/// # Parameters
/// * `arms` - Vector of Arm
fn select_highest_arm_index(arms: &Vec<Arm>) -> usize {
    if arms.len() == 1 {
        0
    } else {
        let mut highest_arm_index: usize = 0;
        for i in 0..arms.len() - 1 {
            if arms[i].calc_success() < arms[i + 1].calc_success() {
                highest_arm_index = i + 1;
            }
        }
        highest_arm_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play() {
        let mut test_arm = Arm::new(0.5);
        let result = test_arm.play();
        assert!(result == 0 || result == 1);
        assert_ne!(test_arm.success, test_arm.fail);
    }
    #[test]
    fn test_calc_success() {
        let mut test_arm = Arm::new(0.5);
        test_arm.fail = 20;
        test_arm.success = 5;
        assert_eq!(test_arm.calc_success(), 0.2);
    }

    #[test]
    fn test_select_highest_arm_index() {
        let mut test_arm1 = Arm::new(0.1);
        test_arm1.fail = 9;
        test_arm1.success = 1;
        let mut test_arm2 = Arm::new(0.1);
        test_arm2.fail = 8;
        test_arm2.success = 2;
        let mut test_arm3 = Arm::new(0.1);
        test_arm3.fail = 7;
        test_arm3.success = 3;
        let mut test_arm4 = Arm::new(0.1);
        test_arm4.fail = 6;
        test_arm4.success = 4;

        let mut v = vec![test_arm1];
        assert_eq!(select_highest_arm_index(&v), 0);
        v.push(test_arm2);
        assert_eq!(select_highest_arm_index(&v), 1);
        v.push(test_arm3);
        assert_eq!(select_highest_arm_index(&v), 2);
        v.push(test_arm4);
        assert_eq!(select_highest_arm_index(&v), 3);
    }
}

/// main関数
fn main() {
    let count = 10000;
    let arm1 = Arm::new(0.5);
    let arm2 = Arm::new(0.2);
    let arm3 = Arm::new(0.7);
    let mut v = vec![arm1, arm2, arm3];

    println!("Ramdom : {}", ramdom_greedy(&mut v, count));

    let epsilon = 0.4;
    println!("ϵ-greedy : {}", epsilon_greedy(&mut v, count, epsilon));
}
 