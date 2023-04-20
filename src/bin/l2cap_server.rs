use bluer::{
    adv::Advertisement,
    l2cap::{SocketAddr, StreamListener},
    Address, AddressType,
};
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let _adv_handle = adapter
        .advertise(Advertisement {
            discoverable: Some(true),
            ..Default::default()
        })
        .await?;

    let sa = SocketAddr::new(Address::any(), AddressType::LePublic, 240);
    let listener = StreamListener::bind(sa).await?;

    let (mut stream, remote_sa) = listener.accept().await?;
    println!("Connection from {remote_sa:?}");

    loop {
        let mut buf = [0; 256];
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        println!("{}", String::from_utf8_lossy(&buf[..n]));
    }

    Ok(())
}
