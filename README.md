# Rust Proxy for ZMQ and GRPC POC

This is a proof-of-concept for a Rust process that can proxy ZMQ and
GRPC traffic coming into the same port to their respective backend
processes.

There are five binaries in this crate:

* zmq_push
A simple ZMQ PUSH socket client that sends messages.

* zmq_pull
A simple ZMQ PULL socket server that receives messages from `zmq_push`.

* grpc_client
A Tonic-based GRPC client that sends simple GRPC/Protobuf messages.

* grpc_server
A Tonic-based GRPC server that receives messages from `grpc_server`

* proxy
A process that detects whether incoming TCP connections are GRPC or
ZMQ, and then forwards the connections to `zmq_pull` or `grpc_server`,
as appropriate.

## Prerequisites

```sh
brew install cmake # for static compilation of ZMQ
brew install tmuxinator # to run the demo session
```

## Building

```sh
cargo build
```

## Demo

The demo creates a tmux session with two windows; one for servers, and
one for clients.

The server window has panes for `proxy`, `zmq_pull`, and
`grpc_server`. All are running on different ports, and `proxy` is
configured with the respective ports of `zmq_pull` and `grpc_server`.

The client window has four panes total, with two each for separate
`zmq_push` and `grpc_client` processes. Importantly, all clients are
configured to connect to the `proxy` service's port.

To run the demo, simply type:

``` sh
tmuxinator
```

When you are done, the following command will kill the `tmux` session:

```sh
tmux kill-session -t zmq-grpc-proxy-poc
```

## Why?

This was a test to figure out how to replace the ZMQ communication
logic in the [Habitat
Supervisor](https://github.com/habitat-sh/habitat) with GRPC in a
backwards-compatible way.
