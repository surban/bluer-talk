use bluer::{l2cap, AdapterEvent, Address, AddressType, Result};
use futures::{pin_mut, StreamExt};
use tokio::{io::AsyncWriteExt, time::sleep};

#[tokio::main]
async fn main() -> Result<()> {
    let remote = Address::new([0xDC, 0xA6, 0x32, 0xF9, 0xC9, 0xF8]);

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let device_events = adapter.discover_devices_with_changes().await?;
    pin_mut!(device_events);
    while let Some(evt) = device_events.next().await {
        if let AdapterEvent::DeviceAdded(addr) = evt {
            let dev = adapter.device(addr)?;
            if addr == remote && dev.rssi().await?.is_some() {
                println!("Device found");
                break;
            }
        }
    }
    sleep(std::time::Duration::from_millis(1000)).await;

    let sa = l2cap::SocketAddr::new(remote, AddressType::LePublic, 240);

    let mut stream = l2cap::Stream::connect(sa).await?;
    sleep(std::time::Duration::from_millis(1000)).await;

    stream.write_all(b"hello l2cap").await?;
    Ok(())
}
