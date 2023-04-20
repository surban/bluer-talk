use bluer::{l2cap, Address, AddressType, Result};
use tokio::{io::AsyncWriteExt, time::sleep};

#[tokio::main]
async fn main() -> Result<()> {
    let remote = Address::new([0xe4, 0x5f, 0x01, 0x49, 0xdd, 0xd8]);
    let sa = l2cap::SocketAddr::new(remote, AddressType::LePublic, 240);

    let mut stream = l2cap::Stream::connect(sa).await?;
    sleep(std::time::Duration::from_millis(100)).await;

    stream.write_all(b"hello l2cap").await?;
    Ok(())
}
