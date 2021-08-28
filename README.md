# Planetoid

Planetoid is a toy project to demonstrate and learn several technologies.
The goal is to create a little multiplayer [asteriod](https://en.wikipedia.org/wiki/Asteroids_(video_game)) game clone.
* Server side is composed with 2 parts:
    * A server based on a [Quarkus](https://quarkus.io/) application. The goal of this application will be to:
      * Show games in progress and terminated with participants and winner.
      * Allow users to authenticate and add comments to a specific game.
      * Launch workers to allow several games in parallel each with individual players.
    * A worker based on a [Quarkus](https://quarkus.io/) application using websockets derived from the chat example. The goal of this application is currently to:
      * Pass game messages between clients.
* Client side is a [Rust](https://www.rust-lang.org/) application using [macroquad](https://github.com/not-fl3/macroquad) framework. As well, it was derived from the asteroid example, but refactored in a more object oriented code. It can be compiled as:
    * A native application that will use websockets ([tungstenite](https://github.com/snapview/tungstenite-rs)) to share game data. Only Linux has been fully tested so far, but it should run on Windows/MacOs as well.
    * A wasm32 application, that can be run in a browser. Currently websockets are not implemented, but the game can be played in solo mode.
* Deployment on [Kubernetes](https://kubernetes.io/) for the server and the required infrastructure to capture metrics ([Prometheus](https://prometheus.io/) / [Grafana](https://grafana.com/)) as well as authentication ([Keycloak](https://www.keycloak.org/)) and persistance ([Postgres](https://www.postgresql.org/)).


This project is in an early stage, so a lot of features are missing and need to be implemented. However, as stated at the beginning, the goal is not to propose a real game but a demo to explain and share about these technologies.


## Targeted infra overview
![infra](images/infra.png)

## Project current status
* Clients (native and wasm) can be built and run. Wasm can only run solo mode.
* Worker allows to play multiplayer game:
    * Native client can share the game with a spectator. Spectator is another native client started in the spectator mode.
    * Multiplayer game. Native client can be run as host and several guests can connect to destroy asteroids together.
* Server is a WIP, this is currently just exposing 2 tables with hibernate/panache and a couple of api routes.


## Authors

- [@Uggla](https://www.github.com/Uggla)

## Game controls
* `Right` and `left` arrow keys to turn the ship right and left.
* `Space` key to shoot.
* `F` key to display fps.
* `Esc` key to quit the game.

## Demo
Online demo can be played [here](https://planetoid.uggla.fr).

*Note:*
* *only solo mode is available online.*
* *loading the game can take time.*

## Screenshots

Native application:
![App native screenshot](images/planetoid_native.jpg)

Running the wasm application into Firefox:
![App wasm32 screenshot](images/planetoid_wasm32.jpg)

Multiplayer game:
![multiplayer game screenshot](images/multiplayer_game.jpg)


## Binaries
Binaries are available here:
[Binary releases](https://github.com/uggla/planetoid/releases)

## Run Locally (mainly for development purpose)

1. Clone the project

```bash
  git clone https://github.com/uggla/planetoid
```

2. Go to the project directory

```bash
  cd planetoid
```

### Worker

1. Install OpenJDK 11 following the instructions [here](https://adoptopenjdk.net/installation.html#) or install it using your distribution package manager.
Ex on Fedora

```bash
  dnf install java-11-openjdk-devel
```

2. Install maven > 3.6 following the instructions [here](https://maven.apache.org/install.html) or install it using your distribution mackage manager. Ex on Fedora:

```bash
  dnf install maven
```

3. Go to the worker directory and run the worker in dev mode

```bash
cd worker
mvn compile quarkus:dev
```
*Note: maven will download a lot of dependencies from the internet*

### Client

#### Native client
1. Install Rust following the instructions [here](https://www.rust-lang.org/fr/learn/get-started).

   *Tips: the rustup method is the simplest method.*

2. Install required library for macroquad

* Ubuntu system dependencies
```bash
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

* Fedora system dependencies
```bash
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel
```

* Windows system
```
No dependencies are required for windows
```

3. Go to the client directory and run the native client
```bash
cd client
cargo run
```

#### Wasm32 client

1. Follow the above instruction of the native client.

2. Install basic-http-server
```bash
cargo install basic-http-server
```

3. Add the wasm32 compilation target
```bash
rustup target add wasm32-unknown-unknown
```

4. Go to the client directory and run the native client
```bash
cd client
cargo build --target wasm32-unknown-unknown
```

5. Serve the files and open the browser
```bash
basic-http-server
xdg-open http://127.0.0.1:4000
```

<!-- ## Installation -->

<!-- Install my-project with npm -->

<!-- ```bash -->
<!--   npm install my-project -->
<!--   cd my-project -->
<!-- ``` -->


## Native client usage
```
Planetoid 0.1.0
Planetoid is a asteroid clone

USAGE:
    planetoid [FLAGS] [OPTIONS]

FLAGS:
    -d, --debug      Debug mode (_ (error), -d (info), -dd (debug), -ddd (trace))
    -g, --god        God mode
        --help       Prints help information
    -s, --solo       Solo mode, do not connect to network
    -V, --version    Prints version information

OPTIONS:
    -h, --host <host>    Host [default: localhost]
    -m, --mode <mode>    Network mode [default: host]  [possible values: host, guest, spectator]
    -n, --name <name>    Player name [default: planetoid]
    -p, --port <port>    Port [default: 8080]
```

### Examples
#### Running in solo mode
`cargo run -- -s`

#### Running in network mode with a spectator
On the first terminal:
`cargo run -- -m host -n Planetoid`

On the second terminal:
`cargo run -- -m spectator -n "Planetoid spectator"`

#### Running in network mode, debug and as god
`-dd`: debug allow to see messages sent to the web socket.

`-g`: god mode, ship cannot be destroyed.

`-n`: player name (default: planetoid)

`cargo run -- -m host -dd -g -n Planetoid`

#### Running in network mode with host and guest
On the first terminal:
`cargo run -- -m host -n Planetoid`

On the second terminal:
`cargo run -- -m guest -n "Planetoid guest"`
