[English](README_EN.md)

# jump-jump 微信跳一跳游戏
- [x] 角色跳跃动画
- [x] 角色和平台蓄力效果
- [x] 角色蓄力粒子特效
- [x] 角色摔落效果
- [x] 相机跟随
- [x] 自动生成平台
- [x] 飘分效果
- [x] 分数统计
- [x] 游戏UI
- [x] 游戏音效
- [x] WASM支持

在线游玩：[点这里](https://nightswatchgames.github.io/games/jump-jump/)（电脑版Chrome/Firefox/Edge打开）

## 运行
1. 本地运行
```
cargo run
```
2. WASM运行
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

## 游戏展示
视频演示：[B站](https://www.bilibili.com/video/BV1B24y1q7aW)

![main menu](https://raw.githubusercontent.com/NightsWatchGames/jump-jump/master/screenshots/main_menu.png)
![game playing](https://raw.githubusercontent.com/NightsWatchGames/jump-jump/master/screenshots/game_playing.png)
![game over](https://raw.githubusercontent.com/NightsWatchGames/jump-jump/master/screenshots/game_over.png)

## 参考
- https://github.com/yaoshanliang/weapp-jump
- https://github.com/wswei99/tiaoyitiao
- https://github.com/potato47/jump