use anyhow::{Context, Result};
use iroh::{endpoint::presets, Endpoint, EndpointAddr, EndpointId};
use std::str::FromStr;

const ALPN: &[u8] = b"calemity/private-transport-spike/1";

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("host") => run_host().await,

        Some("connect") => {
            let endpoint_id = args
                .next()
                .context("Usage: cargo run -- connect <ENDPOINT_ID>")?;

            run_client(&endpoint_id).await
        }

        _ => {
            println!("Calemity private transport spike");
            println!();
            println!("Host:");
            println!("  cargo run -- host");
            println!();
            println!("Connect:");
            println!("  cargo run -- connect <ENDPOINT_ID>");

            Ok(())
        }
    }
}

fn private_endpoint_builder() -> iroh::endpoint::Builder {
    Endpoint::builder(presets::N0)
        // Calemity privacy experiment:
        //
        // Remove IP-based peer transports entirely.
        // The endpoint must communicate using a relay
        // transport rather than creating a direct
        // peer-to-peer IP path.
        .clear_ip_transports()
}

async fn run_host() -> Result<()> {
    let endpoint = private_endpoint_builder()
        .alpns(vec![ALPN.to_vec()])
        .bind()
        .await
        .context("Could not create private Iroh endpoint")?;

    println!("Waiting for relay connectivity...");

    endpoint.online().await;

    println!();
    println!("Calemity private endpoint is online.");
    println!();
    println!("Endpoint ID:");
    println!("{}", endpoint.id());
    println!();
    println!("Waiting for a connection...");

    let incoming = endpoint
        .accept()
        .await
        .context("Endpoint closed before connection arrived")?;

    let connection = incoming.await.context("Connection handshake failed")?;

    let (mut send, mut receive) = connection
        .accept_bi()
        .await
        .context("Could not accept stream")?;

    let data = receive
        .read_to_end(64 * 1024)
        .await
        .context("Could not read message")?;

    let message = String::from_utf8(data).context("Received invalid UTF-8")?;

    println!();
    println!("Received:");
    println!("{message}");

    send.write_all(b"Hello back through the Calemity privacy spike!")
        .await
        .context("Could not send response")?;

    send.finish().context("Could not finish response")?;

    connection.closed().await;
    endpoint.close().await;

    Ok(())
}

async fn run_client(endpoint_id: &str) -> Result<()> {
    let endpoint_id = EndpointId::from_str(endpoint_id).context("Invalid endpoint ID")?;

    let endpoint = private_endpoint_builder()
        .bind()
        .await
        .context("Could not create private Iroh endpoint")?;

    endpoint.online().await;

    println!("Connecting through relay transport...");

    let address = EndpointAddr::from(endpoint_id);

    let connection = endpoint
        .connect(address, ALPN)
        .await
        .context("Could not connect")?;

    let (mut send, mut receive) = connection
        .open_bi()
        .await
        .context("Could not open stream")?;

    send.write_all(b"Hello through Calemity's private transport!")
        .await
        .context("Could not send message")?;

    send.finish().context("Could not finish request")?;

    let data = receive
        .read_to_end(64 * 1024)
        .await
        .context("Could not read response")?;

    let response = String::from_utf8(data).context("Response was invalid UTF-8")?;

    println!();
    println!("Received:");
    println!("{response}");

    connection.close(0u32.into(), b"done");
    endpoint.close().await;

    Ok(())
}
