#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

use bls12_381;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    // let mut rand_bytes = [0_u32; 32];
    // let randomness = env::syscall(SYS_RANDOM, &[], rand_bytes.as_mut_slice());
    // let scalar = Scalar::from_bytes_be(randomness);
    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}
