use bluer::{id, AdapterEvent, Device};
use futures::{pin_mut, StreamExt};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let device_events = adapter.discover_devices_with_changes().await?;
    pin_mut!(device_events);

    while let Some(device_event) = device_events.next().await {
        if let AdapterEvent::DeviceAdded(addr) = device_event {
            let device = adapter.device(addr)?;
            if device
                .uuids()
                .await?
                .unwrap_or_default()
                .contains(&id::Service::HealthThermometer.into())
            {
                test_device(device).await?;
                break;
            }
        }
    }

    Ok(())
}

async fn test_device(device: Device) -> bluer::Result<()> {
    println!("Testing device {}", device.address());

    if !device.is_connected().await? {
        device.connect().await?;
    }

    for service in device.services().await? {
        if service.uuid().await? != id::Service::HealthThermometer.into() {
            continue;
        }

        let mut temperature = None;
        let mut heater_on = None;
        for char in service.characteristics().await? {
            if char.uuid().await? == id::Characteristic::Temperature.into() {
                temperature = Some(char);
            } else if char.uuid().await? == id::Characteristic::DigitalOutput.into() {
                heater_on = Some(char);
            }
        }
        let temperature = temperature.unwrap();
        let heater_on = heater_on.unwrap();

        heater_on.write(&[0]).await?;
        sleep(Duration::from_millis(100)).await;
        println!(
            "Temperature with heater off is {:?}",
            temperature.read().await?
        );
        heater_on.write(&[1]).await?;
        sleep(Duration::from_millis(100)).await;
        println!(
            "Temperature with heater on is {:?}",
            temperature.read().await?
        );
    }

    Ok(())
}
