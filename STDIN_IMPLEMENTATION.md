# G-CLI Stdin Implementation

This document describes the changes made to add stdin support to G-CLI, enabling interactive command input.

## Overview

This implementation adds stdin support to allow G-CLI to work as an interactive REPL (Read-Eval-Print Loop), similar to how the Claude Code CLI works. Users can now type commands interactively and send them to LabVIEW in real-time.

## Use Case

**Before (one-shot mode):**
```bash
$ g-cli my-app.vi -- add 5 3
Result: 8
[exits]
```

**After (interactive mode with stdin):**
```bash
$ g-cli my-app.vi
> add 5 3
Result: 8
> multiply 10 2
Result: 20
> exit
```

## Architecture

### Message Flow

```
User types in terminal
    ↓
stdin_loop reads line
    ↓
ActionMessage::StdinInput(line)
    ↓
action_loop receives message
    ↓
MessageToLV::Stdin("CMND", line)
    ↓
TCP connection to LabVIEW
    ↓
LabVIEW receives "CMND" message
```

## Files Changed

### 1. `rust-proxy/src/comms.rs`

**Changes:**
- Added `Stdin(String)` variant to `MessageToLV` enum (line 206)
- Added "CMND" message ID handling in `to_buffer()` (lines 217, 228)
- Added test `stdin_command_message_to_buffer()` (lines 387-400)
- Added `try_clone()` method to `AppConnection` (lines 147-155)

**Purpose:**
- Defines the protocol for sending stdin commands to LabVIEW
- Message format: `[length]["CMND"][command string]`
- Allows cloning connection for multi-threaded use

### 2. `rust-proxy/src/stdin_loop.rs` *(NEW FILE)*

**Purpose:**
- Background thread that reads from stdin line-by-line
- Non-blocking with stop signal support
- Sends each line as `ActionMessage::StdinInput` to action loop

**Key features:**
- Reads complete lines (waits for Enter key)
- Handles EOF gracefully (Ctrl+D/Ctrl+Z)
- Ignores empty lines
- Cooperative shutdown via stop signal

### 3. `rust-proxy/src/action_loop.rs`

**Changes:**
- Added `StdinInput(String)` variant to `ActionMessage` enum (line 14)
- Updated imports to include `AppConnection` and `MessageToLV` (line 1)
- Modified `run()` to accept `AppConnection` parameter (line 51)
- Added handler for `StdinInput` messages (lines 88-96)

**Purpose:**
- Receives stdin input from stdin_loop
- Sends it to LabVIEW via TCP connection
- Handles errors in stdin transmission

### 4. `rust-proxy/src/main.rs`

**Changes:**
- Added `mod stdin_loop;` declaration (line 8)
- Clone connection before distributing to threads (lines 76-79)
- Start stdin_loop thread (line 96)
- Pass connection to `action_loop.run()` (line 100)
- Updated comments to reflect 4 concurrent threads (lines 81-86)

**Purpose:**
- Wire all components together
- Manage connection ownership between threads

## Protocol Details

### New Message Type: "CMND"

**Direction:** Rust → LabVIEW

**Format:**
```
[4 bytes: total length][4 bytes: "CMND"][N bytes: command string]
```

**Example:**
```
Input: "add 5 3"
Bytes: \x00\x00\x00\x0B C M N D a d d   5   3
       ^^^^^^^^^^^ ^^^^^^^ ^^^^^^^^^^^^^^^^^^^^^
       length=11   msg ID  content
```

### Existing Messages (unchanged)

**To LabVIEW:**
- `ARGS` - Command line arguments (initial startup)
- `CCWD` - Current working directory

**From LabVIEW:**
- `OUTP` - Stdout output
- `SERR` - Stderr output
- `EXIT` - Exit with code
- `OFLS` - Flush buffers

## Threading Model

The application now runs **4 concurrent threads**:

1. **Main/Action Loop**: Processes all incoming messages, writes to stdout/stderr, sends stdin to LabVIEW
2. **Comms Loop**: Reads messages from LabVIEW over TCP
3. **Stdin Loop**: Reads from stdin line-by-line
4. **Signal Loop**: Handles Ctrl+C (SIGINT)

All threads communicate via channels (message passing) and cooperatively shut down via shared `AtomicBool` stop signal.

## LabVIEW Side Implementation

To receive stdin commands in LabVIEW, you need to:

1. **Handle the "CMND" message type** in your VI's message parser
2. **Parse the command string** (e.g., split by spaces to get function name + args)
3. **Execute the command** using your existing function dispatcher
4. **Send output back** using existing stdout (`OUTP`) mechanism

### Example LabVIEW Pseudocode

```
Case Structure on message ID:
  "ARGS": Handle initial arguments (existing)
  "CCWD": Handle working directory (existing)
  "CMND": [NEW]
    - Split string by spaces
    - First element = function name
    - Remaining elements = arguments
    - Call function dispatcher
    - Send result via stdout
```

## Testing

### Manual Testing

1. Build the Rust code:
   ```bash
   cd rust-proxy
   cargo build
   ```

2. Run with a test VI that handles "CMND" messages:
   ```bash
   g-cli test.vi
   ```

3. Type commands interactively:
   ```
   > help
   > add 5 3
   > exit
   ```

### Unit Tests

Run the message serialization test:
```bash
cd rust-proxy
cargo test stdin_command_message_to_buffer
```

## Design Decisions

### Line-Based vs Raw Input

This implementation uses **line-based** input (waits for Enter key), which is simpler and covers most use cases. As discussed in [Issue #187](https://github.com/G-CLI/G-CLI/issues/187), a raw byte-streaming mode could be added later if needed.

**Pros of line-based:**
- Simple to implement
- Natural for command-line interfaces
- Works well with LabVIEW's string processing
- No special buffering needed

**Future enhancement:** Add `--stdin-raw` flag for character-by-character streaming

### Connection Cloning

The TCP connection is cloned using `TcpStream::try_clone()`, creating two file descriptors to the same socket:
- One handle for reading (comms_loop)
- One handle for writing (action_loop)

This is safe because:
- Reading and writing don't interfere
- Each thread has independent buffers
- No shared mutable state

### Error Handling

Errors in stdin transmission cause graceful shutdown with exit code -1, ensuring the user is notified if communication fails.

## Backward Compatibility

✅ **Fully backward compatible**

- Existing one-shot mode with `--` args still works
- LabVIEW VIs that don't handle "CMND" messages simply ignore stdin
- No breaking changes to existing protocol messages

## Future Enhancements

1. **Optional stdin mode**: Add flag like `--stdin` to enable/disable stdin reading
2. **Raw mode**: Add `--stdin-raw` for byte-by-byte streaming
3. **Prompt customization**: Allow configuring the prompt string
4. **History support**: Add readline-style command history (up/down arrows)
5. **Tab completion**: Integrate with LabVIEW to provide command completion

## Related Issues

- [G-CLI/G-CLI#187](https://github.com/G-CLI/G-CLI/issues/187) - stdin support discussion

## For Pull Request

This implementation keeps changes minimal and well-organized:
- **1 new file** (`stdin_loop.rs`) - easy to review in isolation
- **Clear separation of concerns** - each module has a single responsibility
- **Follows existing patterns** - similar structure to `comms_loop.rs`
- **Well-documented** - comments explain the why, not just the what
- **Tested** - includes unit test for message serialization

All changes maintain the existing code style and architecture patterns.
