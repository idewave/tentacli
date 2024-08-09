# TentaCLI
TentaCLI is embeddable extendable console client for WoW 3.3.5a server.

You can use the CLI to debug TCP packets from/to World of Warcraft Server. Or implement some handlers to send your packet to the server.
To extend existing functionality you can implement own features (see `Feature` trait and `src/features` for examples).
Since tentacli is designed to be used as both a library and an application, you can include it as a library in your own app.

### How to start
+ Run `cargo run`
+ After the first run, **tentacli** will create **Config.yml** and **.env** files in the location specified by `RunOptions { ..., config_path, dotenv_path }`.
+ Edit **Config.yml** to set your account name, password, realm and character for auto-selection (or leave them empty to select manually). You can use regex for name and realm fields.
+ Edit **.env** to set your WoW server's IP address/hostname or keep `127.0.0.1` for a local server
+ Run `cargo run` again

### Features
+ Handles authentication (without reconnection).
+ Parses update packets (both as object and as json), chat, movement and some other packets.
+ Provides a UI with keyboard interaction, including history scrolling and detailed output.
+ Supports automatic realm and character selection (configurable in **Config.yml**)
+ Accepts external features (refer to Feature and src/features for guidance on implementing your own)
+ Supports multiple configurations (you can specify custom paths for **Config.yml** and **.env**)
+ Supports multiple accounts per host (configurable in **Config.yml**)
+ Automatically creates a character if the account is empty (configurable in **Config.yml**)

### You want to contribute
It's always welcome. Just create pull request with your improvements, bugfix etc.

### Want to discuss ?
Join us on Discord: https://discord.gg/2qa6dS3Aj6 !

### If you want to support...
...you could buy me a [ko-fi](https://ko-fi.com/idewave)

### Documentation
I do my best to keep it up-to-date. Check our [Wiki](https://github.com/idewave/tentacli/wiki)