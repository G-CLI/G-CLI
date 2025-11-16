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

### G-CLI -> LabVIEW Command Line Parameters

For executables the command line parameters are:

`-p:[port number]` - Which TCP port to connect back to the command line agent on.

Followed by any additional custom parameters.

## TCP Messaging

### Message Format

Messages during operation use a tagged format with a leading 4 byte length.

Format:
```
| Length (4 bytes) | Message Type (4 char ascii string) | Message Contents (n bytes) |
```

### Message Types to G-CLI

Available message types will be:

| Message Type | ID |Description |
| --- |  | --- |
| Exit | `EXIT` | Exit the process. The message contents are a string of the decimal number. (up to v1.5.2 this was an I32 encoding of the number) |
| StdOut | `OUTP` | Write to the console. The message contents are a string to be written to the console. |
| StdErr | `SERR` | Write to the console on standard error. The message contents are a string to be written to the console. |
| Flush Output | `OFLS` | Flush the console output buffer. |

The maximum length of data payload is 8992 bytes (9000 - overheads). Longer messages should be split across multiple messages.

### Message Types to LabVIEW

These messages are:

| Message Type | ID |Description |
| --- |  | --- |
| ARGS | `ARGS` | With a payload of tab delimited arguments. |
| CCWD | `CCWD` | Working directory as a string. |

These messages are read by the LabVIEW library as soon as the connection is established. 
