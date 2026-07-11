# G-CLI Stdin Testing Guide

Step-by-step guide to test the stdin implementation.

## Prerequisites

### Install Rust (Windows)

1. Download: https://rustup.rs/
2. Run `rustup-init.exe`
3. Choose option 1 (default)
4. Restart your terminal
5. Verify: `cargo --version`

## Testing Steps

### Step 1: Unit Tests (No LabVIEW Required)

These tests verify the message protocol works correctly:

```bash
cd rust-proxy

# Run all tests
cargo test

# Run just the stdin tests
cargo test stdin

# See verbose output
cargo test -- --nocapture
```

**Expected output:**
```
running 3 tests
test comms::tests::stdin_command_message_to_buffer ... ok
test stdin_test::test_stdin_message_format ... ok
test stdin_test::test_multiple_stdin_commands ... ok

test result: ok. 3 passed
```

### Step 2: Build the Executable

```bash
cd rust-proxy

# Build in release mode (optimized)
cargo build --release

# Or build in debug mode (faster compilation, slower runtime)
cargo build
```

**Output location:**
- Release: `rust-proxy/target/release/g-cli.exe`
- Debug: `rust-proxy/target/debug/g-cli.exe`

### Step 3: Test with Mock LabVIEW (No Real LabVIEW Required)

We created a mock LabVIEW program that simulates how LabVIEW will handle commands.

#### Build the mock:
```bash
cd rust-proxy
cargo build --example mock_labview
```

#### Run it:

**Terminal 1** (Mock LabVIEW):
```bash
cd rust-proxy
cargo run --example mock_labview
```

This will output a port number, like:
```
[Mock LabVIEW] Listening on port 3363
```

**Terminal 2** (G-CLI):
```bash
cd rust-proxy
.\target\debug\g-cli.exe --help
```

Wait - we need to connect them differently. Let me fix the mock...

Actually, there's a **simpler way** to test:

### Step 4: Interactive Test with Mock LabVIEW

The mock LabVIEW example is designed to work standalone. Here's how:

```bash
# Build everything
cd rust-proxy
cargo build --release --example mock_labview

# The mock will be at:
# target/release/examples/mock_labview.exe
```

But g-cli expects to LAUNCH the LabVIEW process. So we need to test differently.

## Easier Testing Approach

Let me create a simpler test VI for you to use instead:

### Option A: Use the Integration Tests

The integration tests (`rust-proxy/tests/stdin_test.rs`) already test the full flow:

```bash
cd rust-proxy
cargo test --test stdin_test -- --nocapture
```

This will show:
```
Received message ID: CMND
Received content: test command
✓ Received: add 5 3
✓ Received: multiply 10 2
✓ Received: exit
```

### Option B: Manual Test with Echo Server

Let me create a simple echo server that you can test with:

**File: `rust-proxy/examples/echo_server.rs`** (I'll create this next)

This will:
1. Act like LabVIEW
2. Echo back any CMND it receives
3. Let you test stdin interactively

### Option C: Test with Your LabVIEW VI

Once your VI is ready:

```bash
cd rust-proxy
.\target\release\g-cli.exe path\to\your.vi
```

Then type commands:
```
> help
> add 5 3
> exit
```

## Expected Behavior

### When working correctly:

**You type:**
```
> add 5 3
```

**What happens internally:**
1. stdin_loop reads "add 5 3"
2. action_loop receives it
3. Sends TCP message: `[length]["CMND"]["add 5 3"]` to LabVIEW
4. LabVIEW processes it
5. LabVIEW sends back: `[length]["OUTP"]["Result: 8\n"]`
6. You see: `Result: 8`

### When there's a problem:

**Compile errors:**
- Check you're in `rust-proxy` directory
- Run `cargo clean` then `cargo build`

**Connection errors:**
- LabVIEW didn't start properly
- Check logs with `-v` flag: `g-cli -v your.vi`

**No response to commands:**
- LabVIEW isn't handling "CMND" messages
- Check LabVIEW VI has CMND case

## Debugging

### Enable verbose mode:
```bash
g-cli -v your.vi
```

Shows:
```
[HH:MM:SS.mmm] G CLI Started - Verbose Mode
[HH:MM:SS.mmm] Stdin received: add 5 3
[HH:MM:SS.mmm] Sending stdin to LabVIEW: add 5 3
```

### Check if stdin thread is running:

In the logs, you should see:
```
Stdin Loop thread started
```

### Test message serialization:

```bash
cargo test stdin_command_message_to_buffer -- --nocapture
```

Should show:
```
Message bytes: [0, 0, 0, 11, 67, 77, 78, 68, 97, 100, 100, 32, 53, 32, 51]
                 ^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                 length   "CMND"      "add 5 3"
```

## Next Steps

1. ✅ Install Rust
2. ✅ Run unit tests: `cargo test`
3. ✅ Build g-cli: `cargo build --release`
4. ⏳ Prepare your LabVIEW VI to handle "CMND" messages
5. ⏳ Test with your VI: `g-cli your.vi`
6. ⏳ Debug if needed with `-v` flag

## If You Get Stuck

Common issues:

**"cargo: command not found"**
- Restart terminal after installing Rust
- Check: `echo %PATH%` includes `.cargo\bin`

**"failed to compile"**
- Share the error message
- Might be a typo in the code changes

**"connection timeout"**
- LabVIEW didn't start
- Check if your VI path is correct
- Try with existing working VI first

**stdin not working**
- Test with integration tests first: `cargo test stdin_test`
- Enable verbose mode: `-v`
- Check LabVIEW has CMND handler

Good luck! 🚀
