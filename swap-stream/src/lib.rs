use solana_sdk::pubkey::Pubkey;
use anchor_client::solana_client::rpc_client::RpcClient;
use std::fmt;

#[derive(Debug)]
pub enum QuoteError {
    QuoteNotFound
}

#[derive(Debug)]
pub enum RouteError {
    UnableToRoute
}

#[derive(Debug)]
pub enum SwapError {
    UnableToSwap
}


// TODO add in keypair to wallet
/// Configruation for swaps
pub struct SwapStream {
    /// Instance of rpc client for sending transactions
    connection: RpcClient,
    /// Global allowed slippage
    slippage: f64,
}


impl SwapStream {
    /// Create a new instance of swapstream
    pub fn new(connection: RpcClient, slippage: f64) -> Self {
        Self {
            connection,
            slippage
        }
    }


    /// Compute the best dex to swap between base and quote
    pub fn quote(&self, base: String, quote: String) -> Result<Quote, SwapError> {
        todo!()
    }

    /// Combine all quotes into a valid route
    pub fn route(&self, quotes: Vec<Quote>) -> Result<Route, RouteError> {
        todo!()
    }

    /// Peform the swap specified by the computed route
    pub fn swap(&self, route: Route) -> Result<Route, SwapError> {
        todo!()
    }

}



/// The best quote between the base and quote tokens
pub struct Quote {
    /// The base token of the swap
    base: Pubkey,
    // The quote token of the swap
    quote: Pubkey,
    /// Amount of base to swap in
    amount_in: f64,
    /// Amount of quote to get out
    amount_out: f64,
    /// How much slippage allowed on the swap
    slippage: f64,
    /// The route for this swap
    route: Route,
}




pub struct Route {
    /// The swaps to perform for this route
    swaps: Vec<Swap>
}

pub struct Swap {
    /// Identifier of the amm
    amm_id: Pubkey,
    /// The base token of the swap
    base: Pubkey,
    /// The quote token of the swap
    quote: Pubkey,
    /// Amount of base to swap in
    amount_in: f64,
    /// Amount of quote to get out
    amount_out: f64,
}
