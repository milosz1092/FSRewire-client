<img src="https://github.com/milosz1092/FSRewire-client/blob/main/static/window_icon.png?raw=true" alt="FSRewire-logo" style="width:20px;height:20px;" />[![Release](https://github.com/milosz1092/FSRewire-client/actions/workflows/release.yaml/badge.svg)](https://github.com/milosz1092/FSRewire-client/actions/workflows/release.yaml)

# FSRewire-client

This application serves as a discovery service for identifying **Microsoft Flight Simulator** host.

## Features

It facilitates unattended connection setup with the SimConnect server from remote devices.

 - **SimConnect Configuration** 📝: Updates the SimConnect configuration to enable remote communication.
 - **Discovery Service** 📢: Utilizes UDP broadcasting for disseminating connection information.
 - **User Interface** 📇: Offers a minimalistic user interface for monitoring the application's status.
 - **System Tray Integration** 📥: Integrates with the system tray, allowing users to access essential functions conveniently.

## Usage
 1. Execute the compiled binary to launch the FSRewire-client application.
 2. Run Microsoft Flight Simulator alongside.
 3. UDP packets with connection details will be emitted through your local network.
    - The IP address of the host can be retrieved from the packet itself.
    - Datagram content format: `FSR_SMC:{SimConnectPort}` (for example: `FSR_SMC:500`).

## How it works?

```
   FSRewire-client                                       External Software
   +-----------------------+                             +-----------------------+
   |                       |                             |                       |
   | 1. Discovery Service  |                             | 4. Connect to Host    |
   |                       |                             |                       |
   +-----------+-----------+                             +-----------+-----------+
               |                                                     ^
               |                                                     |
               v                                                     |
   +-----------+-----------+                             +-----------+-----------+
   |                       |                             |                       |
   | 2. Broadcast Host     |                             | 3. Receive Host Data  |
   |    Information        |                             |                       |
   |                       |                             |                       |
   +-----------------------+                             +-----------------------+
```

## Legend:

#### FSRewire-client (Left Side):
   1. Discovery Service:
      - FSRewire-client acts as a Discovery Service within the local network.
   2. Broadcast Host Information:
      - FSRewire-client periodically broadcasts packets containing host information (IP address, SimConnect port) to the local network.

#### External Software (Right Side):
   3. Receive Host Data:
      - External software listens for broadcasted packets and retrieves the host information (IP address, port) sent by FSRewire-client.
   4. Connect to Host:
      - External software running on another device within the local network attempts to connect to the host announced by FSRewire-client.
