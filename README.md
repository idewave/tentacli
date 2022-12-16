# idewave-cli
This is Rust implementation of World of Warcraft client v3.3.5a. Smart CLI.

You can use the CLI to debug TCP packets from/to World of Warcraft Server. Or send your own (need to implement handlers).
Also, you can use it as bot (but you need to implement actions by yourself, see MovementAI for example).

### How to start
+ Rename **Config.yml.dist** into **Config.yml**, then edit it according to your preferences.
+ Rename **.env.example** into *.env* and edit it according to your preferences.
+ Run command: cargo run -r

### Features
+ Authentication (without reconnection)
+ Update packets, chat, movement
+ UI with keyboard interaction (including history scrolling and details output in DEBUG mode)

### You want to contribute
It's always welcome. Just create pull request with your improvements, bugfix etc.