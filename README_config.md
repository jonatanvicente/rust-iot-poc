

### Configuration detail

* User to execution at RPi: `iotuser`
* Service name: `iot-bridge`

#### Service creation to start the server at boot time

```
[Unit]
Description=Rust IIoT Gateway (MQTT + HTTP)
# Wait until the network is ready and Mosquitto is running
After=network.target mosquitto.service
Requires=mosquitto.service

[Service]
# The user you created earlier for security
User=iotuser
Group=iotuser

# The path to your compiled gateway binary
ExecStart=/usr/local/bin/iot-bridge/gateway

# If it crashes, wait 5 seconds and try again
Restart=always
RestartSec=5

# Helps with logging
StandardOutput=journal
StandardError=journal

[Install]
# This tells Linux to start the service in normal multi-user mode
WantedBy=multi-user.target
```

After creating the file, check the status:

```bash
sudo systemctl daemon-reload
sudo systemctl enable iot-bridge
sudo systemctl start iot-bridge
sudo systemctl status iot-bridge
```

Also you can check the logs with:

```bash
sudo journalctl -u iot-bridge -f
```

