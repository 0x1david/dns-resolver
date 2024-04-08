mod message;

use crate::message::message::{AsBytes, Message};
use anyhow::{Context, Result};
use clap::command;
use clap::Parser;
use log::{debug, error, info};
use tokio::{self, net::UdpSocket};

#[derive(Parser, Debug)]
#[command(name = "DNS Forwarder")]
#[command(version = "1.0")]
#[command(about = "A simple DNS forwarder", long_about = None)]
struct Args {
    #[arg(short, long)]
    resolver: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    debug!("Main started with args: {:?}", args);

    let udp_socket = UdpSocket::bind("127.0.0.1:2053")
        .await
        .expect("Failed to bind to address");
    info!("DNS server started on 127.0.0.1:2053");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf).await {
            Ok((size, source)) => {
                debug!(
                    "Received {} bytes from {}: {:?}",
                    size,
                    source,
                    &buf[..size]
                );

                let request = Message::parse_request(&buf);
                let response = if !args.resolver.is_empty() {
                    info!("Querying resolver");
                    match resolve_query(&args.resolver, request).await {
                        Ok(r) => r,
                        Err(e) => panic!("{}", e),
                    }
                } else {
                    info!("Creating local response.");
                    request.create_response().as_bytes()
                };
                match udp_socket.send_to(&response, source).await {
                    Ok(bytes_sent) => {
                        debug!("Sent {} bytes in response to {}", bytes_sent, source)
                    }
                    Err(e) => error!("Failed to send response: {}", e),
                }
            }
            Err(e) => {
                error!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

async fn resolve_query(addr: &str, query: Message) -> Result<Vec<u8>> {
    info!("Resolving Query");
    let request_buf = query.split_as_bytes();
    let mut response_buf = vec![0u8; 512];
    let mut response = query.create_answerless_response().as_bytes();

    info!("Binding Socket");
    let socket = UdpSocket::bind("localhost:0")
        .await
        .context("Failed binding to the resolver UdpSocket")?;
    socket
        .connect(addr)
        .await
        .context("Failed connectiong to the resolver UdpSocket")?;

    for r in request_buf {
        socket.send(&r).await.context("Failed sending query")?;
        let len = socket
            .recv(&mut response_buf)
            .await
            .context("Error receiving response")?;
        response_buf.truncate(len);

        let rsp = Message::parse_resolver_response(&response_buf).answer;
        rsp.iter().for_each(|answer| {
            response.append(&mut answer.as_bytes());
        });
    }

    info!("Returning response after resolution finished");
    Ok(response)
}
