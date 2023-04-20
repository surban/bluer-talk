use bluer::{Adapter, AdapterEvent, Address};
use futures::{pin_mut, StreamExt};

#[tokio::main]
async fn main() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let device_events = adapter.discover_devices_with_changes().await?;
    pin_mut!(device_events);

    while let Some(device_event) = device_events.next().await {
        match device_event {
            AdapterEvent::DeviceAdded(addr) => {
                println!("Device added / changed: {addr}");
                let _ = query_device(&adapter, addr).await;
            }
            AdapterEvent::DeviceRemoved(addr) => {
                println!("Device removed: {addr}");
            }
            _ => (),
        }
    }

    Ok(())
}

async fn query_device(adapter: &Adapter, addr: Address) -> bluer::Result<()> {
    let device = adapter.device(addr)?;
    if device.name().await?.as_deref() != Some("le_advertise") {
        return Ok(());
    }
    println!("  Address type: {}", device.address_type().await?);
    println!("  Name:         {:?}", device.name().await?);
    println!("  Connected:    {:?}", device.is_connected().await?);
    println!("  Paired:       {:?}", device.is_paired().await?);
    println!("  Trusted:      {:?}", device.is_trusted().await?);
    println!("  RSSI:         {:?}", device.rssi().await?);
    println!("  TX power:     {:?}", device.tx_power().await?);
    println!("  Services:     {:?}", device.uuids().await?.unwrap_or_default());
    println!("  Service data: {:?}", device.service_data().await?);
    Ok(())
}
