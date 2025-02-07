# TentaCLI
Tentacli is a headless (like a tentacle) console client for World of Warcraft server (supported version: 3.3.5a).
By default, it serves as a tool for displaying real-time TCP traffic in human-readable format (JSON).
It is adapted for scenarios where you need to run multiple clients simultaneously 
and supports data sharing between instances, making it suitable for stress testing your server.

You can extend its functionality by creating your own plugins (features), 
allowing it to act as a bot or anything else you need. Additionally, it can be integrated into your own app as a crate.

Here are some examples of how the default UI looks:
![Image](https://github.com/user-attachments/assets/f803139a-eaf7-47d7-acf0-8d783f6f4da6)

![Image](https://github.com/user-attachments/assets/45c81ff7-832b-43aa-92d1-2a25b4a3bdf4)

However, you are not required to use this UI (or any UI at all), as it is possible to implement your own plugin (feature) to replace it.

### How to start

+ Run `cargo run`, so **Config.yml** and **.env** files will be created in the location specified by `config_path` and `dotenv_path` in `RunOptions { ..., config_path, dotenv_path }`.
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
+ Packet processors are provided as a separate feature, allowing you to implement your own packet processors and handlers.

### You want to contribute
It's always welcome. Just create pull request with your improvements, bugfix etc.

### If you want to support...
...you can buy me a [ko-fi](https://ko-fi.com/idewave)

### Documentation
I do my best to keep it up-to-date. Have a look at the project's [Wiki](https://github.com/idewave/tentacli/wiki)
