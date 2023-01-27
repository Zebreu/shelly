"Fish Game" for Macroquad
=====================

![fishgame](https://user-images.githubusercontent.com/910977/114977971-304e2f00-9e4e-11eb-83ed-707abb895466.gif)


**"Fish Game" for Macroquad** is an online multiplayer game, created as a
demostration of [Nakama](https://heroiclabs.com/), an open-source scalable game
server, using [Rust programming language](https://www.rust-lang.org/) and
the [Macroquad](https://github.com/not-fl3/macroquad/) game engine.

Playing the game online
----------------------------

The latest web build for online play is available [here](https://fedorgames.itch.io/fish-game?secret=UAVcggHn332a).

Playing the game from source
----------------------------

Depedencies:

The main depdency: the rust compiler.   
To get it, follow [rustup.rs](https://rustup.rs/) instructions.

On web, windows and mac os no other external depdendecies are required.
On linux followed libs may be required: 
```
apt install libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

### Nakama server

To run the Fish game locally Nakama server is required.

The easiest way to setup a Nakama server locally for testing/learning purposes is [via Docker](https://heroiclabs.com/docs/install-docker-quickstart/), and in fact, there is a `docker-compose.yml` included in the source code of "Fish Game" (/docker/docker-compose.yml).

So, if you have [Docker Compose](https://docs.docker.com/compose/install/) installed on your system, all you need to do is navigate to "/docker" directory and run:

```
docker-compose up
```

This will automatically pull all Fish game .lua modules and will gives a ready to connect nakama server. 

### Running the game:

### Native PC build: 

```
cargo run --release
```
from this repo root.

### Building HTML5 build:

```
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/fishgame.wasm web/fishgame.wasm
wasm-strip web/fishgame.wasm
```

To serve the web build some web server will be required. One of the options: [devserver](https://github.com/kettle11/devserver) 

```
cargo install devserver
cd web
devserver .
```

And than open `http://localhost:8080`
