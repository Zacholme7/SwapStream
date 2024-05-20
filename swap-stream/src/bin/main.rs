// everything here is single step

// example execution
//use swap_stream::{SwapStream, route, swap, quote};
use anchor_client::solana_client::rpc_client::RpcClient;
use swap_stream::SwapStream;

fn main() {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com");

    // construct the swapper with your wallet, rpc connection, and allowed slippage
    // todo! add in the wallet
    let swapper = SwapStream::new(client, 0.05);

    // construct the quotes
    let sol_bonk = swapper.quote("SOL".to_string(), "BONK".to_string()).unwrap();
    let bonk_sol = swapper.quote("SOL".to_string(), "BONK".to_string()).unwrap();


    // aggregate the quotes
    let route = swapper.route(vec![sol_bonk, bonk_sol]).unwrap();

    // perform the swap
    let swap = swapper.swap(route).unwrap();
}

