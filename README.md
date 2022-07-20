# idewave-cli
This is Rust implementation of World of Warcraft client v3.3.5a. Smart CLI.

Use this project to customize actions when interact with WoW server. Interaction can be automatic and manual. 

In case of automatic interaction you just define any amount of handlers and bind them to specific opcode inside special `Processor` object. So this handlers will be processed once packet with specific opcode received from server.

In case of manual interaction you can define special manager (see **movement/ai** as example) where you decide by yourself when to send packets (or do another actions).

### How to start
Rename **Config.yml.dist** into **Config.yml**, then edit it according to your preferences.
Please notice regex currently supported only for `realm_name` param (see no case why to support somewhere else).

Then just run:

> cargo run

### What implemented already
1. Full authentication flow (without extra checks)
2. Parser of update packets
3. Examples of chat/movement handling
4. Incomplete warden part
5. Support of yml config

*Unfortunately, I have no experience with reverse engineering*. 
If somebody can help to finish warden part I would appreciate it.
Please consider also: I do not support cheaters and cheat making. 
This part implemented in learning purpose only.

### You want contribute
It's always welcome. Just create pull request with your improvements, bugfix etc.

If you want to support me you can buy me a coffee on https://ko-fi.com/idewave.

Please join us on discord: https://discord.gg/PvaJ2g4zTp
