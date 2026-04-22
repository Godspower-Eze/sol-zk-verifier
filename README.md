# Solana ZK Verifier

A zero-knowledge proof verifier for Solana that implements Groth16 verification on-chain.

## Overview

This program enables verification of Groth16 zero-knowledge proofs directly on the Solana blockchain. It validates cryptographic proofs against a pre-configured verifying key, allowing trustless verification of computations without revealing the underlying data.

## Building

Build the program:
```bash
anchor build
```

## Testing

Run the test suite with release optimizations:
```bash
cargo test --release
```

This executes the Groth16 proof verification test using the Solana test validator (LiteSVM).

## Deployed Contracts

### Devnet
- **Program ID**: `5STsA74ZCLZXGdv33vP5fk1EgLBE4HNR494UpiqXXDz`
- **Network**: Solana Devnet
- **Status**: ✅ Deployed (Slot 457244444)
- **Verify**: `solana program show 5STsA74ZCLZXGdv33vP5fk1EgLBE4HNR494UpiqXXDz --url devnet`

## Technical Details

- **Framework**: Anchor
- **Language**: Rust
- **Curve**: BN254
- **Proof System**: Groth16
- **Dependencies**: `groth16-solana` (Light Protocol)

## How It Works

The program accepts:
1. **Proof bytes** (256 bytes): Contains proof components (A, B, C)
2. **Public inputs** (N × 32-byte values): The public parameters of the proof where N is number of public inputs for the circuit

It performs BN254 elliptic curve arithmetic to verify the proof validity against the hardcoded verifying key.
