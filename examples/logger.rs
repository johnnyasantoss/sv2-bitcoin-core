use std::path::Path;
use sv2_bitcoin_core::Sv2BitcoinCore;

use tokio_util::sync::CancellationToken;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // the user must provide the path to the Bitcoin Core UNIX socket
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <bitcoin_core_unix_socket_path>", args[0]);
        eprintln!("Example: {} /path/to/bitcoin/regtest/node.sock", args[0]);
        std::process::exit(1);
    }

    let bitcoin_core_unix_socket_path = Path::new(&args[1]);
    let cancellation_token = CancellationToken::new();
    let tokio_local_set = tokio::task::LocalSet::new();

    let coinbase_output_max_additional_size = 1;
    let coinbase_output_max_additional_sigops = 1;

    let cancellation_token_clone = cancellation_token.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("Ctrl+C received");
        cancellation_token.cancel();
    });

    tokio_local_set
        .run_until(async move {
            let sv2_bitcoin_core = Sv2BitcoinCore::new(
                bitcoin_core_unix_socket_path,
                cancellation_token_clone.clone(),
                coinbase_output_max_additional_size,
                coinbase_output_max_additional_sigops,
            )
            .await
            .unwrap();

            sv2_bitcoin_core.run().await;
        })
        .await;
}
