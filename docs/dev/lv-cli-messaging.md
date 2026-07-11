# G CLI Messaging Protocol

So that LabVIEW can communicate with the CLI, we use a simple text based protocol over TCP as well
as some means to connect through TCP in the first place.

## Establishing a TCP Connection

To establish a TCP connection we need to negotiate a port number to connect back to.

This is primarily done through the NI Service Locator but can be done over the command line for executables.

### NI Service Locator

The NI Service Locator is used to find the port number to connect back to.

G-CLI will register a port based on the launch path of the VI with the service locator.
The LabVIEW library will then look up the port number for the VI it is running in.

Both the main channel port and the signal channel port are registered - the
signal channel is published under `<id>/signal`, where `<id>` is the same
`cli/<version>/<bitness>/<vi path>` ID used for the main channel
(`rust-proxy/src/labview/port_discovery.rs::Registration::register`).

The Service Locator lookup only works when LabVIEW is **not** set to allow
multiple instances of the VI (`Check If Multiple LV Instances Allowed.vi`) -
the registration ID is derived only from the VI path/version/bitness, so it
can't disambiguate between several concurrently running instances of the
same VI. When multiple instances are allowed, LabVIEW instead reads the port
directly from the `-p:`/`-p2:` command line parameters below, since each
launched instance gets its own values there. G-CLI always does both
(registers with the Service Locator *and* passes the command line
parameters) regardless of launch type, so LabVIEW can use whichever
mechanism applies.

### G-CLI -> LabVIEW Command Line Parameters

The command line parameters are:

`-p:[port number]` - Which TCP port to connect back to the command line agent on (the main channel).

`-p2:[port number]` - Which TCP port to connect back to for the signal channel (see below).

Followed by any additional custom parameters.

## TCP Messaging

G-CLI opens **two** TCP connections to LabVIEW:

- The **main channel** (`-p:`) carries console I/O and process lifecycle
  messages (`ARGS`, `CCWD`, `OUTP`, `SERR`, `EXIT`, `OFLS`).
- The **signal channel** (`-p2:`) carries stdin data and other asynchronous
  signals bound for LabVIEW (`STIN`, `SIGI`). It is kept separate from the
  main channel so code that only wants to react to stdin doesn't have to sit
  in a case structure filtering out unrelated message types - see
  [issue #187](https://github.com/G-CLI/G-CLI/issues/187) for the discussion
  that drove this. LabVIEW is expected to connect to both ports as soon as it
  starts, and to run an asynchronous read loop on the signal channel for the
  lifetime of the process (see `docs/dev/lv-signal-channel.md` for a VI-level
  spec of that loop).

### Message Format

Messages during operation use a tagged format with a leading 4 byte length.
The same framing is used on both the main and signal channels.

Format:
```
| Length (4 bytes) | Message Type (4 char ascii string) | Message Contents (n bytes) |
```

### Message Types to G-CLI

Available message types will be (sent on the main channel):

| Message Type | ID |Description |
| --- |  | --- |
| Exit | `EXIT` | Exit the process. The message contents are a string of the decimal number. (up to v1.5.2 this was an I32 encoding of the number) |
| StdOut | `OUTP` | Write to the console. The message contents are a string to be written to the console. |
| StdErr | `SERR` | Write to the console on standard error. The message contents are a string to be written to the console. |
| Flush Output | `OFLS` | Flush the console output buffer. |

The maximum length of data payload is 8992 bytes (9000 - overheads). Longer messages should be split across multiple messages.

### Message Types to LabVIEW

On the **main channel**:

| Message Type | ID |Description |
| --- |  | --- |
| ARGS | `ARGS` | With a payload of tab delimited arguments. |
| CCWD | `CCWD` | Working directory as a string. |

These messages are read by the LabVIEW library as soon as the connection is established.

On the **signal channel**:

| Message Type | ID | Description |
| --- | --- | --- |
| Stdin | `STIN` | A chunk of stdin data. Sent either when a full line (including trailing `\n`) has been read, or after a 100ms timeout with whatever partial/unterminated input has accumulated (so piped input without a trailing newline is still delivered promptly). See `rust-proxy/src/stdin_loop.rs`. |
| Signal | `SIGI` | Ctrl+C / SIGINT notification ([issue #188](https://github.com/G-CLI/G-CLI/issues/188)). Sent (empty payload) the moment g-cli catches Ctrl+C, *before* it force-kills the LabVIEW process - see `rust-proxy/src/signal_loop.rs`. LabVIEW gets `--ctrlc-timeout` milliseconds (default 60000, `-1` for indefinite) to react and exit on its own before g-cli kills it. Requires the signal channel to be enabled (default; disabled with `--no-signal`). |
