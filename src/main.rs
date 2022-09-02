mod uniswapv2;

#[tokio::main]
async fn main() {
    uniswapv2::uniswap_v2_balance().await.ok();
}
