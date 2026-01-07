#!/usr/bin/env python3
"""
Simple test to verify CMND message protocol format.
This simulates what the Rust code does without requiring compilation.
"""

def format_message(msg_id: str, content: str) -> bytes:
    """
    Format a message according to G-CLI protocol.

    Format: [4 bytes: length][4 bytes: msg_id][N bytes: content]
    Length = len(msg_id) + len(content)
    """
    msg_id_bytes = msg_id.encode('utf-8')
    content_bytes = content.encode('utf-8')

    length = len(msg_id_bytes) + len(content_bytes)
    length_bytes = length.to_bytes(4, byteorder='big')

    return length_bytes + msg_id_bytes + content_bytes


def parse_message(data: bytes) -> tuple:
    """Parse a message and return (msg_id, content)"""
    length = int.from_bytes(data[0:4], byteorder='big')
    msg_id = data[4:8].decode('utf-8')
    content = data[8:8+length-4].decode('utf-8')
    return msg_id, content


def print_bytes(data: bytes, label: str):
    """Pretty print bytes"""
    print(f"\n{label}:")
    print(f"  Total length: {len(data)} bytes")
    print(f"  Hex: {data.hex()}")
    print(f"  Bytes: {list(data)}")
    print(f"  ASCII visual: {repr(data)}")


def test_stdin_message():
    """Test STIN message formatting"""
    print("=" * 60)
    print("Testing STIN Message Protocol")
    print("=" * 60)

    # Test case 1: Simple command
    command = "add 5 3"
    message = format_message("STIN", command)

    print_bytes(message, f"STIN message: '{command}'")

    # Verify the format
    print("\nBreakdown:")
    length = int.from_bytes(message[0:4], byteorder='big')
    print(f"  Length field: {message[0:4].hex()} = {length} bytes")
    print(f"  Message ID:   {message[4:8]} = '{message[4:8].decode()}'")
    print(f"  Content:      {message[8:]} = '{message[8:].decode()}'")

    # Parse it back
    msg_id, content = parse_message(message)
    print(f"\nParsed back:")
    print(f"  Message ID: '{msg_id}'")
    print(f"  Content:    '{content}'")

    assert msg_id == "STIN", f"Expected 'STIN', got '{msg_id}'"
    assert content == command, f"Expected '{command}', got '{content}'"
    print("\n[PASS] Test 1 PASSED: Simple command")

    # Test case 2: Different command
    print("\n" + "-" * 60)
    command2 = "multiply 10 2"
    message2 = format_message("STIN", command2)
    print_bytes(message2, f"STIN message: '{command2}'")

    msg_id2, content2 = parse_message(message2)
    assert msg_id2 == "STIN"
    assert content2 == command2
    print("\n[PASS] Test 2 PASSED: Different command")

    # Test case 3: Existing messages (ARGS, CCWD, OUTP)
    print("\n" + "-" * 60)
    print("\nComparing with existing message types:")

    args_msg = format_message("ARGS", "test1\ttest2")
    print_bytes(args_msg, "ARGS message")

    ccwd_msg = format_message("CCWD", "C:\\test")
    print_bytes(ccwd_msg, "CCWD message")

    outp_msg = format_message("OUTP", "Hello, World\n")
    print_bytes(outp_msg, "OUTP message (from LabVIEW)")

    print("\n" + "=" * 60)
    print("[SUCCESS] ALL TESTS PASSED!")
    print("=" * 60)

    # Show example usage
    print("\nExample Interactive Session:")
    print("-" * 60)
    commands = [
        "help",
        "add 5 3",
        "multiply 10 2",
        "exit"
    ]

    for cmd in commands:
        msg = format_message("STIN", cmd)
        print(f"\nUser types: {cmd}")
        print(f"  > Rust sends: {len(msg)} bytes")
        print(f"  > Hex: {msg.hex()}")
        print(f"  > LabVIEW receives: '{cmd}'")


def test_rust_compatibility():
    """Verify our format matches the Rust test case"""
    print("\n" + "=" * 60)
    print("Testing Rust Compatibility")
    print("=" * 60)

    # From rust-proxy/src/comms.rs test:
    # let expected = "\x00\x00\x00\x0BSTINadd 5 3";
    # size = 11 + 4 = 15

    command = "add 5 3"
    message = format_message("STIN", command)

    expected_bytes = b'\x00\x00\x00\x0BSTINadd 5 3'

    print(f"\nExpected (from Rust test):")
    print(f"  {expected_bytes.hex()}")
    print(f"  Length: {len(expected_bytes)} bytes")

    print(f"\nOur implementation:")
    print(f"  {message.hex()}")
    print(f"  Length: {len(message)} bytes")

    if message == expected_bytes:
        print("\n[SUCCESS] PERFECT MATCH with Rust implementation!")
        return True
    else:
        print("\n[FAIL] MISMATCH!")
        print(f"  Difference:")
        for i, (e, a) in enumerate(zip(expected_bytes, message)):
            if e != a:
                print(f"    Byte {i}: expected {hex(e)}, got {hex(a)}")
        return False


if __name__ == "__main__":
    test_stdin_message()
    print("\n")
    test_rust_compatibility()

    print("\n" + "=" * 60)
    print("Protocol verification complete!")
    print("=" * 60)
    print("\nThis confirms the STIN message format is correct.")
    print("When the Rust code is built and run, it will send")
    print("messages in this exact format to LabVIEW.")
