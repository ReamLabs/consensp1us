# consensp1us

`consensp1us` is a Proof of Concept of integrating zkVM with the current Ethereum Consensus layer. It will leverage [ream](https://github.com/ReamLabs/ream) and [sp1](https://github.com/succinctlabs/sp1).

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/getting-started/install.html)

## Running the Project

There are four main ways to run this project: build a program, execute a program, generate a core proof, and
generate an EVM-compatible proof.

### Build the Program

To build the program, run the following command:

```sh
cd program/operations
cargo prove build
```

### Execute the Program

To run the program without generating a proof:

```sh
cd script
make download
cargo run --release -- --execute --operation-name <OPERATION_NAME> --ef-test
```

```
possible values for OPERATION_NAME: attestation, attester_slashing, block_header, bls_to_execution_change, deposit, execution_payload, proposer_slashing, sync_aggregate, voluntary_exit, withdrawals
```

This will execute the program and display the output.

### Generate benchmarks for execution

```sh
cd script
make download
make run-<OPERATION_NAME>
```

```sh
OPERATIONS = attestation attester_slashing block_header bls_to_execution_change deposit execution_payload proposer_slashing sync_aggregate voluntary_exit withdrawals
```

This will execute the program and generate benchmarks (especially for cycles) in `./script/summaries` directory.

### Generate a Core Proof

To generate a core proof for your program:

```sh
cd script
cargo run --release -- --prove --operation-name <OPERATION_NAME> --ef-test
```
