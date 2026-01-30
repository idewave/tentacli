## Tentacli

Tentacli is a framework for exploring how network protocols work, extended through plugins.

It runs as a client application that connects directly to a server (or multiple servers), reads and sends packets according to defined rules, and presents the protocol in a human-readable form.

Tentacli is not a sniffer or a MITM tool.  
It does not intercept third-party traffic — it participates in the protocol as a full-fledged endpoint.

On top of Tentacli, you can build automated clients that share a common context.  
The context (`anymap2`) is shared across all plugins and connections, used to store protocol state and processing logic, and is available for both read and write access at runtime (`Arc<RwLock<CtxMap>>`).

Tentacli can be used, for example:
- as a protocol visualization tool
- as a testbed for servers using controlled clients
- as a foundation for specialized protocol clients

The architecture does not lock you into a fixed set of use cases — what you build is defined by the plugins you write and the packet processing logic you attach.

![Image](https://github.com/user-attachments/assets/9f753516-2a2c-41fe-865f-b92dbc963a41)

# History

Tentacli began as a console-based World of Warcraft client, built to study protocol behavior from the client side.

As development progressed, recurring patterns became clear:

- connection management
- packet framing and parsing
- protocol semantics
- event propagation
- UI and automation

These patterns were extracted into a generic, protocol-agnostic runtime.  
The result is a framework capable of hosting multiple independent TCP/UDP connections, each extended through modular, reusable plugins.

---

## How Tentacli Is Structured

The core is a runtime responsible for launching and coordinating plugins.  
There are three types of plugins:

---

### Network plugin

Used to establish a connection to a server and define how the protocol is handled.

Responsible for:
- establishing the connection to the server
- selecting the transport (TCP / UDP)
- reading the raw byte stream from the socket
- extracting packets from the incoming buffer
- serializing packets for transmission

Internally, it spawns two asynchronous tasks:
- `read_task` — reads data from the socket, accumulates a buffer, and extracts packets
- `write_task` — receives packets from the core and sends them to the server

Each network plugin operates under its own label (`ServerLabel`).  
Labels must be unique: registering two network plugins with the same label will cause a panic at startup.

---

### Processor plugin

Extends a specific network plugin.

It does not participate in the connection itself. Instead, it provides:
- parsers for specific packet types
- groups of protocol logic handlers
- generators for outgoing packets and requests
- logic for reading from and modifying the shared context

Processor plugins attach to a network plugin by matching its `ServerLabel` and are inserted into the shared packet processing pipeline.

---

### Core plugin

Acts as the system coordinator.

A core plugin:
- receives signals from all processor plugins
- can send control signals and responses back to network plugins
- can implement higher-level logic for routing and handling those signals

It does not work with raw bytes or protocol data directly, but operates on events and processing results (`HandlerOutput`, `Echo`, `Requests`).

---

## Doctor mode

Tentacli provides a built-in diagnostic mode to inspect the current build and runtime environment:

```bash
cargo run --no-default-features --features <your features> -- doctor

# or

./tentacli doctor
```

It reports:
- Enabled build features
- Registered plugins (network / processor / core)
- Wiring between processors and network plugins
- Configuration resolution context (env, OS config dir, working directory)
- This is useful for debugging misconfigured builds, missing plugins, or unexpected config resolution.

## Want to Contribute? 

Contributions are always welcome. Feel free to open a pull request with improvements, bug fixes, or new plugins.

At the moment, a network plugin for **WoW WotLK** is implemented.

If this project speaks to you, I’d be glad for any help in building new plugins for other gaming and application-level protocols — network, processor, and core plugins alike.

## Join us in Discord

Before joining, please read the rules: https://discord.gg/tgEdFD8V22 !

## License

This project is licensed under the Apache License 2.0.
See the LICENSE and NOTICE files for details.
