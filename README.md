# idewave-cli
This is Rust implementation of smart wow client (cli). 

This project provide structure for easy apply any amount of handlers of user actions when interact with wow server.
Current supported version of wow is **3.3.5a** only.

Packet interaction contains of two parts: processors part and manual part. 
Processor is just automatic reaction to packets that come from server (like response to LOGIN_CHALLENGE).
Manual part currently implemented only within AI part (see movement/ai).

In the future I want to improve some I/O and make manual part more extended.

### How to start
Rename **Config.yml.dist** into **Config.yml**, then edit it according to your preferences.
Please notice regex currently supported only for `realm_name` param (see no case why to support somewhere else).

Then just run:

> cargo run

### What implemented already
1. Full authentication flow (need to add checks if user banned etc)
2. Parser of update packets
3. Examples of chat/movement handling
4. Incomplete warden part
5. Support of yml config

*Unfortunately, I have no experience with reverse engineering*. 
If somebody can help to finish warden part I would appreciate it.
Please consider also: I do not support cheaters and cheat making. 
This part implemented in learning purpose only.

### In case you want contribute
It's always welcome. There incomplete part of warden interaction. 
A lot of opcodes need to support.

If you want to support me you can buy me a coffee on https://ko-fi.com/idewave.

Please join us on discord: https://discord.gg/PvaJ2g4zTp