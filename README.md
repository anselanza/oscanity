# OSCanity
Sanity for OSC. See what I did there?

![oscanity in action](./docs/oscanity-demo.gif)

OSCanity is a command-line utility for testing sending and receiving [OSC](http://opensoundcontrol.org/) messages.

## Usage
### Running the executables
If installed, e.g. using `cargo install oscanity`, then you can simply run `oscanity receive` and `oscanity send` commands. Append `--help` at the end for command-line options.

### Receiving
Example command - listen for incoming OSC messages:
```
oscanity receive
```
This would listen on all network interfaces (`0.0.0.0` on the port `12345`). Substitute your own values as needed, e.g. `--port 3333`.

### Sending
#### Configure
Example command - send messages OSC messages to localhost:
```
oscanity send
```
This sets up a default "sending" socket bound to `127.0.0.1:54321`, and sends OSC messages to the destination `127.0.0.1:12345`. You typically only need to specify a different target `--host` and `--port` (i.e. destination port of the receiving end).


#### Now, send messages
The sending CLI does *nothing* at first (apart from validating any address settings).

Type messages in the format `/address arg1 arg2 ...` followed by Enter/Return.

OSC types for messages are auto-detected. For example, the command:
```
/test hello 1 2 3.0 four
```
... will produce an OSC message with the address `/test` followed by 5 arguments, each with the expected type:
```
final args: [String("hello"), Int(1), Int(2), Float(3.0), String("four")]
```
