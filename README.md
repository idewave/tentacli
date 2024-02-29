# TentaCLI
TentaCLI is embeddable, extendable console client for WoW 3.3.5a server.

You can use the CLI to debug TCP packets from/to World of Warcraft Server. Or even send your own packet 
(need to implement handlers).
Also, you can use it as bot (but you need to implement actions by yourself and attach as separate feature, for examples you could check lib.rs rustdoc or existing features like ui or console).

Or even integrate with your own app (by [installing it with cargo](https://crates.io/crates/tentacli))

### How to start
+ After first run tentacli will create Config.yml and .env files in the location that
you passed to the RunOptions { ..., config_path, dotenv_path }.
+ Edit **Config.yml**: set your account name, password, 
realm and character for autoselect (or set them empty to select manually)
+ Edit **.env**: set your Wow Server IPADDR/HOST or keep 127.0.0.1 for local server
+ Run again

### Features
+ Authentication (without reconnection)
+ Parses update packets, chat, movement and some other basic stuff
+ UI with keyboard interaction (including history scrolling and details output in DEBUG mode)
+ Auto Realm/Character, configurable in Config.yml
+ Accepts external feature set
+ Supports multi-config (you can pass custom config and .env paths)
+ Supports multi-account sets (you can set multiple account per host in config)
+ Supports auto-character create on empty or newly added accounts (see config)

### You want to contribute
It's always welcome. Just create pull request with your improvements, bugfix etc.

### Want to discuss ?
Join us on Discord: https://discord.gg/2qa6dS3Aj6 !

### If you want to support...
...you could buy me a [ko-fi](https://ko-fi.com/idewave)