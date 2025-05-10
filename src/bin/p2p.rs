use std::{error::Error, str::FromStr};

use futures::StreamExt;
use libp2p::{core::multiaddr::Multiaddr, identify, noise, swarm::SwarmEvent, tcp, yamux};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            identify::Behaviour::new(identify::Config::new(
                "/ipfs/id/1.0.0".to_string(),
                key.public(),
            ))
        })?
        .build();

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    // swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    let id = swarm.local_peer_id();
    println!("Id: {}", id);

    swarm
        .listen_on("/dnsaddr/bootstrap.libp2p.io".parse::<Multiaddr>()?)
        .unwrap();
    println!("Dialed");

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            // Prints peer id identify info is being sent to.
            SwarmEvent::Behaviour(identify::Event::Sent { peer_id, .. }) => {
                println!("Sent identify info to {peer_id:?}")
            }
            // Prints out the info received via the identify event
            SwarmEvent::Behaviour(identify::Event::Received { info, .. }) => {
                println!("Received {info:?}")
            }
            other => {
                println!("Received swarm event: {:#?}", other);
            }
        }
    }
}
