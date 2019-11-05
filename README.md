# OSCanity
Sanity for OSC. See what I did there?

## Usage
### Receiving
Example command - listen for incoming OSC messages:
```
cargo run --bin receive 0.0.0.0:12345
```
This would listen on all network interfaces (`0.0.0.0` on the port `12345`). Substitute your own values (especially the port) as needed.

### Sending
#### Configure
Example command - send messages OSC messages to localhost:
```
cargo run --bin send 127.0.0.1:12345
```
This sets up a default "sending" host of `127.0.0.1:8080`, and sends OSC messages to the destination `127.0.0.1:12345` (in this case localhost).

If you need it, you can include the host address as well:
```
cargo run --bin send 192.168.1.130:5555 127.0.0.1:12345
```
Now the "sending" host is `192.168.1.130:555` and the destination is as before.

#### Now, send messages
The sending CLI does *nothing* at first (apart from validating any address settings).

Type messages in the format `address/ arg1 arg2 ...` followed by Enter/Return.

OSC types for messages are auto-detected. For example, the command:
```
/test hello 1 2 3.0 four
```
... will produce an OSC message with the address `/test` followed by 5 arguments, each with the expected type:
```
final args: [String("hello"), Int(1), Int(2), Float(3.0), String("four")]
```

## Background
This is my first working software written in Rust, so be nice.

The hard stuff is done by the library I am depending on here, [rosc](https://github.com/klingtnet/rosc) - "an implementation of the OSC 1.0 protocol in pure Rust".
