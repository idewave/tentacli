# TentaCLI
TentaCLI is embeddable, extendable console client for WoW 3.3.5a server.

You can use the CLI to debug TCP packets from/to World of Warcraft Server. Or send your own (need to implement handlers).
Also, you can use it as bot (but you need to implement actions by yourself, see MovementAI for example).
Or even integrate with your own app

### How to start
+ Rename **Config.yml.dist** into **Config.yml**, then edit it according to your preferences.
+ Rename **.env.example** into *.env* and edit it according to your preferences.
+ Run command: cargo run -r

You can also install it as library from cargo.

### Features
+ Authentication (without reconnection)
+ Update packets, chat, movement
+ UI with keyboard interaction (including history scrolling and details output in DEBUG mode)
+ Auto Realm/Character, configurable in Config.yml
+ Accepts external broadcast channel and external feature set

### You want to contribute
It's always welcome. Just create pull request with your improvements, bugfix etc.

### Want to discuss ?
Join us on Discord: https://discord.gg/wcqXekEvE6 !