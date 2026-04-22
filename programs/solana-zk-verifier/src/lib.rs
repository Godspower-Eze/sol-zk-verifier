pub mod constants;
pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;

pub use constants::N;
pub use instructions::*;

declare_id!("AR9BVLEzUAUHK7Fq4SjXoRkE7mCSR2qnZHQDUCkUYruQ");

#[program]
pub mod solana_zk_verifier {
    use crate::constants::VERIFYING_KEY;

    use super::*;
    use ark_bn254;
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
    type G1 = ark_bn254::g1::G1Affine;
    use groth16_solana::groth16::Groth16Verifier;
    use std::ops::Neg;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn verify(
        ctx: Context<Verify>,
        proof_bytes: Vec<u8>,
        public_inputs: [[u8; 32]; N],
    ) -> Result<()> {
        let proof_a: G1 = G1::deserialize_with_mode(
            &*[&change_endianness(&proof_bytes[0..64]), &[0u8][..]].concat(),
            Compress::No,
            Validate::Yes,
        )
        .unwrap();
        let mut proof_a_neg = [0u8; 65];
        proof_a
            .neg()
            .x
            .serialize_with_mode(&mut proof_a_neg[..32], Compress::No)
            .unwrap();
        proof_a
            .neg()
            .y
            .serialize_with_mode(&mut proof_a_neg[32..], Compress::No)
            .unwrap();
        let proof_a_vec = change_endianness(&proof_a_neg[..64]);
        let proof_a_arr: [u8; 64] = proof_a_vec.try_into().expect("invalid length");
        let proof_b = proof_bytes[64..192].try_into().unwrap();
        let proof_c = proof_bytes[192..256].try_into().unwrap();
        let mut verifier = Groth16Verifier::new(
            &proof_a_arr,
            proof_b,
            proof_c,
            &public_inputs,
            &VERIFYING_KEY,
        )
        .unwrap();
        verifier.verify().unwrap();
        verifier.verify_unchecked().unwrap();
        Ok(())
    }
}

fn change_endianness(bytes: &[u8]) -> Vec<u8> {
    let mut vec = Vec::new();
    for b in bytes.chunks(32) {
        for byte in b.iter().rev() {
            vec.push(*byte);
        }
    }
    vec
}
