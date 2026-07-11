# LabVIEW Stdin Implementation Guide

Quick guide for implementing stdin command handling in your LabVIEW VI.

## Message Format

When you type a command in the terminal (e.g., `add 5 3`), g-cli sends:

```
Message ID: "STIN"
Content: "add 5 3"
```

## What You Need to Add to Your VI

### 1. Update Your Message Case Structure

Add a new case to handle "CMND" messages:

```
Case Structure (Message ID String):
  "ARGS": [existing] Handle initial command-line arguments
  "CCWD": [existing] Handle current working directory
  "STIN": [NEW] Handle stdin input
     ↓
     Parse command string
     Execute function
     Send result to stdout
```

### 2. Command Parser

For a command like `"add 5 3"`:

1. **Split by spaces** → `["add", "5", "3"]`
2. **First element** = function name (`"add"`)
3. **Rest** = arguments (`["5", "3"]`)

### 3. Example LabVIEW Block Diagram Logic

```
[STIN message content] → [Split String by space]
    ↓
["add", "5", "3"]
    ↓
[Index Array - element 0] → function_name = "add"
[Index Array - rest]      → args = ["5", "3"]
    ↓
[Case Structure on function_name]
    "add":     Call Add.vi with args
    "multiply": Call Multiply.vi with args
    "exit":    Send EXIT message
    else:      Send error to stderr
```

### 4. Sending Output Back

Use your **existing stdout mechanism** (the one that already works):

```
[Your Result] → [Format as string] → [Send via "OUTP" message]
```

## Simple Example VI Flow

```
┌─────────────────────────────────────────┐
│ While Loop (TCP Message Receiver)       │
│                                         │
│  Read TCP Message                       │
│    ↓                                    │
│  Parse Message → ID + Content           │
│    ↓                                    │
│  Case Structure (ID):                   │
│    ┌──────────────────────────────┐    │
│    │ "STIN":                       │    │
│    │   Split content by space      │    │
│    │   Get function name (index 0) │    │
│    │   Get arguments (rest)        │    │
│    │                                │    │
│    │   Case on function_name:      │    │
│    │     "add":                     │    │
│    │       num1 = arg[0]           │    │
│    │       num2 = arg[1]           │    │
│    │       result = num1 + num2    │    │
│    │       Send "Result: {result}" │    │
│    │                                │    │
│    │     "multiply":                │    │
│    │       num1 = arg[0]           │    │
│    │       num2 = arg[1]           │    │
│    │       result = num1 * num2    │    │
│    │       Send "Result: {result}" │    │
│    │                                │    │
│    │     "exit":                    │    │
│    │       Send EXIT message (0)   │    │
│    │                                │    │
│    │     default:                   │    │
│    │       Send "Unknown command"  │    │
│    └──────────────────────────────┘    │
│                                         │
│  Continue loop until EXIT               │
└─────────────────────────────────────────┘
```

## Testing Your Implementation

1. **Build** your VI with CMND message handling
2. **Run** g-cli:
   ```bash
   g-cli your-app.vi
   ```
3. **Type commands** interactively:
   ```
   > add 5 3
   Result: 8
   > multiply 10 2
   Result: 20
   > exit
   ```

## Message Protocol Reference

### Messages TO LabVIEW (from g-cli)

| ID | Content | When |
|---|---|---|
| `ARGS` | Tab-separated args | Startup (once) |
| `CCWD` | Working directory path | Startup (once) |
| `STIN` | Stdin input text | Stdin data (continuous, ~100ms intervals) |

### Messages FROM LabVIEW (to g-cli)

| ID | Content | Purpose |
|---|---|---|
| `OUTP` | String | Print to stdout |
| `SERR` | String | Print to stderr |
| `OFLS` | (none) | Flush output buffers |
| `EXIT` | Integer code | Exit g-cli |

## Tips

1. **Reuse your parser**: If you already parse `ARGS`, reuse that logic for `STIN`
2. **Input sent automatically**: g-cli sends stdin data at ~100ms intervals or on newlines
3. **Clean exit**: When LabVIEW sends EXIT, g-cli will close automatically (no need to press Enter!)
4. **Error handling**: Send errors to stderr using `SERR` message
5. **Exit gracefully**: When done, send `EXIT` message with code 0

## Debugging

Enable verbose mode in g-cli to see messages:
```bash
g-cli -v your-app.vi
```

This will show debug output like:
```
[HH:MM:SS.mmm] Stdin received: add 5 3
[HH:MM:SS.mmm] Sending stdin to LabVIEW: add 5 3
```

## Example Commands You Might Support

- `help` - Show available commands
- `add <num1> <num2>` - Add two numbers
- `multiply <num1> <num2>` - Multiply two numbers
- `status` - Show application status
- `exit` - Graceful shutdown

## Next Steps

1. Add "CMND" case to your message parser
2. Implement command dispatcher
3. Test with simple commands
4. Add more commands as needed
5. Consider adding command validation/help

Good luck! 🚀
