#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

extern crate alloc;

// use bls12_381::{G1Projective, Scalar};
use ark_bls12_381::{Fr, G1Projective};
use ark_ec::Group;

use risc0_zkvm::guest::env;
use risc0_zkvm_platform::syscall::nr::SYS_RANDOM;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};

use alloc::vec::Vec;

risc0_zkvm::guest::entry!(main);

fn main() {
    // read the input
    let input: u32 = env::read();

    let mut rand_bytes = [0_u32; 5];
    let randoms = env::syscall(SYS_RANDOM, &[], rand_bytes.as_mut_slice());

    let mut contribution = G1Projective::generator();

    for i in rand_bytes {
        let scalar = Fr::from(i);
        contribution = G1Projective::generator() * scalar;
    }


    let mut compressed_bytes = Vec::new();
    contribution.serialize_compressed(&mut compressed_bytes).unwrap();
    env::commit(&compressed_bytes);
}
