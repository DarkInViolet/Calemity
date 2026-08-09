use anyhow::{Context, Result};
use iroh::{
    endpoint::{presets, IncomingAddr},
    Endpoint, EndpointAddr, EndpointId, TransportAddr,
};
use std::{
    str::FromStr,
    time::{Duration, Instant},
};

const ALPN: &[u8] = b"calemity/private-transport-spike/1";

const WARMUP_SAMPLES: usize = 10;
const BENCHMARK_SAMPLES: usize = 100;

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

    match incoming.remote_addr() {
        IncomingAddr::Relay { url, .. } => {
            println!();
            println!("Transport check: RELAY ✓");
            println!("Relay: {url}");
        }

        IncomingAddr::Ip(addr) => {
            anyhow::bail!("PRIVACY FAILURE: direct IP transport detected: {addr}");
        }

        IncomingAddr::Custom(_) => {
            anyhow::bail!("PRIVACY FAILURE: unexpected custom transport");
        }

        _ => {
            anyhow::bail!("PRIVACY FAILURE: unknown transport type");
        }
    }

    let connection = incoming.await.context("Connection handshake failed")?;

    // Initial message exchange.
    let (mut send, mut receive) = connection
        .accept_bi()
        .await
        .context("Could not accept initial stream")?;

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

    // Benchmark responder.
    let total_samples = WARMUP_SAMPLES + BENCHMARK_SAMPLES;

    println!();
    println!("Waiting for latency benchmark...");

    for _ in 0..total_samples {
        let (mut send, mut receive) = connection
            .accept_bi()
            .await
            .context("Could not accept benchmark stream")?;

        let data = receive
            .read_to_end(16)
            .await
            .context("Could not read benchmark ping")?;

        if data != b"ping" {
            anyhow::bail!("Benchmark protocol error: expected ping");
        }

        send.write_all(b"pong")
            .await
            .context("Could not send benchmark pong")?;

        send.finish()
            .context("Could not finish benchmark response")?;
    }

    println!();
    println!("Benchmark complete.");

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

    if let Some(remote_info) = endpoint.remote_info(endpoint_id).await {
        let mut relay_addresses = 0;
        let mut ip_addresses = 0;
        let mut custom_addresses = 0;
        let mut unknown_addresses = 0;

        for address in remote_info.addrs() {
            match address.addr() {
                TransportAddr::Relay(_) => {
                    relay_addresses += 1;
                }

                TransportAddr::Ip(_) => {
                    ip_addresses += 1;
                }

                TransportAddr::Custom(_) => {
                    custom_addresses += 1;
                }

                _ => {
                    unknown_addresses += 1;
                }
            }
        }

        println!();
        println!("Remote metadata check:");
        println!("  relay addresses: {relay_addresses}");
        println!("  IP addresses: {ip_addresses}");
        println!("  custom addresses: {custom_addresses}");
        println!("  unknown addresses: {unknown_addresses}");

        if ip_addresses > 0 {
            anyhow::bail!("PRIVACY FAILURE: peer IP address was learned");
        }

        if custom_addresses > 0 || unknown_addresses > 0 {
            anyhow::bail!("PRIVACY FAILURE: unexpected remote transport metadata");
        }
    }

    // Initial message exchange.
    let (mut send, mut receive) = connection
        .open_bi()
        .await
        .context("Could not open initial stream")?;

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

    println!();
    println!("Running relay-only latency benchmark...");
    println!("Warm-up samples: {WARMUP_SAMPLES}");
    println!("Measured samples: {BENCHMARK_SAMPLES}");

    let mut samples = Vec::with_capacity(BENCHMARK_SAMPLES);

    for sample_number in 0..(WARMUP_SAMPLES + BENCHMARK_SAMPLES) {
        let start = Instant::now();

        let (mut send, mut receive) = connection
            .open_bi()
            .await
            .context("Could not open benchmark stream")?;

        send.write_all(b"ping")
            .await
            .context("Could not send benchmark ping")?;

        send.finish().context("Could not finish benchmark ping")?;

        let data = receive
            .read_to_end(16)
            .await
            .context("Could not read benchmark pong")?;

        if data != b"pong" {
            anyhow::bail!("Benchmark protocol error: expected pong");
        }

        let elapsed = start.elapsed();

        if sample_number >= WARMUP_SAMPLES {
            samples.push(elapsed);
        }
    }

    samples.sort_unstable();

    println!();
    println!("Calemity private transport benchmark");
    println!("------------------------------------");
    println!("Samples:     {}", samples.len());
    println!("Minimum RTT: {}", format_duration(samples[0]));
    println!("Median RTT:  {}", format_duration(percentile(&samples, 50)));
    println!("P90 RTT:     {}", format_duration(percentile(&samples, 90)));
    println!("P99 RTT:     {}", format_duration(percentile(&samples, 99)));
    println!(
        "Maximum RTT: {}",
        format_duration(samples[samples.len() - 1])
    );

    connection.close(0u32.into(), b"done");
    endpoint.close().await;

    Ok(())
}

fn percentile(samples: &[Duration], percentile: usize) -> Duration {
    let index = ((samples.len() - 1) * percentile) / 100;
    samples[index]
}

fn format_duration(duration: Duration) -> String {
    format!("{:.2} ms", duration.as_secs_f64() * 1000.0)
}
