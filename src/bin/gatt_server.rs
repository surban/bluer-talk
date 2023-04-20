use bluer::{
    adv::Advertisement,
    gatt::local::{
        Application, Characteristic, CharacteristicRead, CharacteristicReadRequest,
        CharacteristicWrite, CharacteristicWriteMethod, CharacteristicWriteRequest, ReqResult,
        Service,
    },
    id,
};
use futures::{future, FutureExt};
use std::sync::atomic::{AtomicBool, Ordering};

#[tokio::main]
async fn main() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let _adv_handle = adapter
        .advertise(Advertisement {
            local_name: Some("heater".to_string()),
            service_uuids: [id::Service::HealthThermometer.into()]
                .into_iter()
                .collect(),
            discoverable: Some(true),
            ..Default::default()
        })
        .await?;

    let app = Application {
        services: vec![Service {
            uuid: id::Service::HealthThermometer.into(),
            primary: true,
            characteristics: vec![
                Characteristic {
                    uuid: id::Characteristic::Temperature.into(),
                    read: Some(CharacteristicRead {
                        read: true,
                        fun: Box::new(|req| read_temperature(req).boxed()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                Characteristic {
                    uuid: id::Characteristic::DigitalOutput.into(),
                    write: Some(CharacteristicWrite {
                        write: true,
                        write_without_response: true,
                        method: CharacteristicWriteMethod::Fun(Box::new(|value, req| {
                            write_heater_on(value, req).boxed()
                        })),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }],
        ..Default::default()
    };
    let _app_handle = adapter.serve_gatt_application(app).await?;

    future::pending().await
}

static HEATER_ON: AtomicBool = AtomicBool::new(false);

async fn read_temperature(_req: CharacteristicReadRequest) -> ReqResult<Vec<u8>> {
    if HEATER_ON.load(Ordering::SeqCst) {
        Ok(vec![60])
    } else {
        Ok(vec![20])
    }
}

async fn write_heater_on(value: Vec<u8>, _req: CharacteristicWriteRequest) -> ReqResult<()> {
    let on = value[0] != 0;
    println!("Heater switches {}", if on { "on" } else { "off" });
    HEATER_ON.store(on, Ordering::SeqCst);
    Ok(())
}
