pub use {
    crossbeam_channel::Sender, solana_sdk::pubkey::Pubkey, std::net::SocketAddr,
    tokio::sync::oneshot::Sender as OneShotSender,
};

pub trait RequestResponse {
    type Response;
    fn num_expected_responses(&self) -> u32;
    fn verify_response(&self, response: &Self::Response) -> bool;
}
