use num_bigint::{BigInt, BigUint};
use num_traits::{Zero, One};
use num_traits::Pow;

const X: i128 = -80_538_738_812_075_974; 
const Y: i128 = 80_435_758_145_817_515;
const Z: i128 = 12_602_123_297_335_631;

fn main() {
    let x: BigInt = X.into();
    let y: BigInt = Y.into();
    let z: BigInt = Z.into();

    let lhs = x.pow(3u32) + y.pow(3u32) + z.pow(3u32);
    println!("{}", lhs);

    let rhs: BigInt = 42.into();
    println!("lhs {}, rhs: {}", lhs, rhs);
}
