# TentaCLI
TentaCLI is embeddable extendable console client for WoW 3.3.5a server.

You can use the CLI to debug TCP packets from/to World of Warcraft Server. Or implement some handlers to send your packet to the server.
To extend existing functionality you can implement own features (see `Feature` trait and `src/features` for examples).
Since tentacli implemented to be either a lib or an app, you can include it as lib into your own app.

### How to start
+ `cargo run`
+ After first run tentacli will create **Config.yml** and **.env** files in the location that
you passed to the RunOptions { ..., config_path, dotenv_path }.
+ Edit **Config.yml**: set your account name, password, 
realm and character for autoselect (or set them empty to select manually). You can use regex here.
+ Edit **.env**: set your Wow Server IPADDR/HOST or keep 127.0.0.1 for local server
+ `cargo run` again

### Features
+ Passes authentication (without reconnection)
+ Parses update packets (both as object and as json), chat, movement and some other packets
+ Provides UI with keyboard interaction (including history scrolling and details output)
+ Supports auto Realm/Character selecting (set this in Config.yml)
+ Accepts external features (see `Feature` and `src/features` for understanding of how to implement your own)
+ Supports multi-config (you can pass custom **Config.yml** and **.env** paths)
+ Supports multi-accounts (you can set multiple account per host in **Config.yml**)
+ Supports auto-character create if account is empty (see **Config.yml**)

### You want to contribute
It's always welcome. Just create pull request with your improvements, bugfix etc.

### Want to discuss ?
Join us on Discord: https://discord.gg/2qa6dS3Aj6 !

### If you want to support...
...you could buy me a [ko-fi](https://ko-fi.com/idewave)

### Documentation
I do my best to keep it up-to-date. Check our [Wiki](https://github.com/idewave/tentacli/wiki)