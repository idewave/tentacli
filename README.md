# idewave-cli
This is Rust implementation of World of Warcraft client v3.3.5a. Smart CLI.

You can use the CLI to debug TCP packets from/to World of Warcraft Server. Or send your own (need to implement handlers).
Also, you can use it as bot (but you need to implement actions by yourself, see MovementAI for example).

### How to start
Rename **Config.yml.dist** into **Config.yml**, then edit it according to your preferences.

Rename **.env.example** into *.env* and edit it according to your preferences.

Then just run:

> cargo run

### What implemented already
1. Full authentication flow (without extra checks)
2. Parser of update packets
3. Examples of chat/movement handling
4. Incomplete warden part
5. Support of yml config
6. Added UI with keyboard interaction

### You want to contribute
It's always welcome. Just create pull request with your improvements, bugfix etc.