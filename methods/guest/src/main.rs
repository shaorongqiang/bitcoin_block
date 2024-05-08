#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use bitcoin::block::Header;
use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

fn main() {
    let headers: Vec<(u64, Header)> = env::read();

    let ret = if headers.len() > 1 {
        for i in 1..headers.len() - 1 {
            match (headers.get(i - 1), headers.get(i), headers.get(i + 1)) {
                (Some((prev_height, prev_header)), Some((curr_height, curr_header)), None) => {
                    prev_header.validate_pow(prev_header.target()).unwrap();
                    curr_header.validate_pow(curr_header.target()).unwrap();
                    if prev_header.block_hash() != curr_header.prev_blockhash {
                        panic!()
                    }
                    if *prev_height != *curr_height - 1 {
                        panic!()
                    }
                }
                (
                    Some((prev_height, prev_header)),
                    Some((curr_height, curr_header)),
                    Some((next_height, next_header)),
                ) => {
                    prev_header.validate_pow(prev_header.target()).unwrap();
                    curr_header.validate_pow(curr_header.target()).unwrap();
                    next_header.validate_pow(next_header.target()).unwrap();
                    if prev_header.block_hash() != curr_header.prev_blockhash {
                        panic!()
                    }

                    if curr_header.block_hash() != next_header.prev_blockhash {
                        panic!()
                    }

                    if *prev_height != *curr_height - 1 {
                        panic!()
                    }

                    if *curr_height != *next_height - 1 {
                        panic!()
                    }
                }

                _ => panic!(),
            };
        }

        headers
            .last()
            .map(|(height, header)| (*height, header.block_hash()))
            .unwrap()
    } else {
        if let Some((height, header)) = headers.first() {
            header.validate_pow(header.target()).unwrap();
            (*height, header.block_hash())
        } else {
            panic!()
        }
    };

    env::commit(&ret);
}
