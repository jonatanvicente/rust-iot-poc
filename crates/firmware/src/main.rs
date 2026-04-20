use anyhow::Result;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi, Configuration, ClientConfiguration};
use rumqttc::{Client, MqttOptions, QoS};
use std::{thread, time::Duration};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    // 1. Setup WiFi
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "YOUR_WIFI_SSID".try_into().unwrap(),
        password: "YOUR_WIFI_PASSWORD".try_into().unwrap(),
        ..Default::default()
    }))?;

    wifi.start()?;
    wifi.connect()?;
    wifi.wait_netif_up()?;
    println!("WiFi Connected!");

    // 2. Setup MQTT
    // Use the RPi IP address you've been testing with
    let mut mqttoptions = MqttOptions::new("esp32_sensor_1", "192.168.1.62", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    // Spawn a thread to keep the MQTT connection alive
    thread::spawn(move || {
        for _ in connection.iter() {
            // This loop handles ACKs and PINGS in the background
        }
    });

    // 3. The Main Loop
    let mut count = 0;
    loop {
        let payload = format!("{{\"device\": \"esp32\", \"value\": {}, \"status\": \"online\"}}", count);

        match client.publish("esp32/sensor", QoS::AtMostOnce, false, payload) {
            Ok(_) => println!("Published: count {}", count),
            Err(e) => println!("Publish failed: {:?}", e),
        }

        count += 1;
        thread::sleep(Duration::from_secs(5));
    }
}