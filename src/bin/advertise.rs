use bluer::adv::{self, Advertisement};
use futures::future;
use std::collections::BTreeMap;
use uuid::uuid;

#[tokio::main]
async fn main() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let mut service_data = BTreeMap::new();
    service_data.insert(
        uuid!("123e4567-e89b-12d3-a456-426614174000"),
        vec![0x1, 0x02, 0x03],
    );

    let advertisement = Advertisement {
        advertisement_type: adv::Type::Peripheral,
        service_data,
        discoverable: Some(true),
        local_name: Some("le_advertise".to_string()),
        ..Default::default()
    };

    let _handle = adapter.advertise(advertisement).await?;
    future::pending().await
}
