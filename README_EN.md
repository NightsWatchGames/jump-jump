# jump-jump
- [x] Jump animation
- [x] Charging animation
- [x] Charging particle effect
- [x] Fall animation
- [x] Camera follow
- [x] Generate platforms
- [x] Score up animation
- [x] Score board
- [x] Game ui
- [x] Game sounds
- [x] WASM support

Play online: [click here](https://nightswatchgames.github.io/games/jump-jump/) (Open with PC Chrome/Firefox/Edge)

## Get started
1. Native
```
cargo run
```
2. WASM
```
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```
```
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/jump-jump.wasm
```

## Screenshots
Game video: [Youtube]()

![main menu](https://raw.githubusercontent.com/NightsWatchGames/jump-jump/master/screenshots/main_menu.png)
![game playing](https://raw.githubusercontent.com/NightsWatchGames/jump-jump/master/screenshots/game_playing.png)
![game over](https://raw.githubusercontent.com/NightsWatchGames/jump-jump/master/screenshots/game_over.png)

## Reference
- https://github.com/yaoshanliang/weapp-jump
- https://github.com/wswei99/tiaoyitiao
- https://github.com/potato47/jump