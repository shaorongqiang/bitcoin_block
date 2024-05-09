use bitcoin::{consensus::serialize, hashes::Hash, BlockHash};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use methods::{BLOCK_GUEST_ELF, BLOCK_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let input = {
        let mut input = Vec::new();
        let url = "http://127.0.0.1:18443";
        let auth = Auth::UserPass("admin1".into(), "123".into());

        let client = Client::new(url, auth).unwrap();

        let begin = 10;
        let end = 13;
        for height in begin..=end {
            let header = client
                .get_block_hash(height)
                .and_then(|hash| client.get_block_header(&hash))
                .unwrap();
            println!("{}", header.block_hash());
            input.extend(serialize(&header));
        }
        input
    };

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let receipt = prover.prove(env, BLOCK_GUEST_ELF).unwrap();

    let ret = receipt.journal.decode::<[u8; 32]>().unwrap();
    println!("output: {}", BlockHash::from_byte_array(ret));

    receipt.verify(BLOCK_GUEST_ID).unwrap();
}
