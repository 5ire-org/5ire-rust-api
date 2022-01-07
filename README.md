# 5ire-rust-api 

* Compose extrinsics, send them and subscribe to updates (synchronously).
* supports composing extrinsics for `no_std` builds
* Watch events and execute code upon events.
* Parse and print the node metadata.

## Prerequisites

In order to build the 5ire-rust-api and the examples, Rust and the wasm target are needed. For Linux:

    curl https://sh.rustup.rs -sSf | sh

    rustup default nightly
    rustup target add wasm32-unknown-unknown --toolchain nightly

For more information, please refer to the [substrate](https://github.com/paritytech/substrate) repository.

## Tutorial

There is a detailed tutorial in the [tutorials](/tutorials) folder.

## Examples

To run an example, clone the `5ire-rust-api` repository and run the desired example directly with the cargo command:

```bash
    git clone https://github.com/5ire-org/5ire-rust-api.git
    cd 5ire-rust-api
    cargo run --example example_get_storage
```

Set the output verbosity by prepending `RUST_LOG=info` or `RUST_LOG=debug`.

The following examples can be found in the [examples](/src/examples) folder:

* [example_simple_connect](/src/examples/example_simple_connect.rs): Connect to a node and print chain info.
* [example_print_metadata](/src/examples/example_print_metadata.rs): Print the metadata of the node in a readable way.
* [example_get_storage](/src/examples/example_get_storage.rs): Read storage values.
* [example_read_storage](/src/examples/example_read_storage.rs): Read storage values.
* [example_read_storage_at_blockhash](/src/examples/example_read_storage_at_blockhash.rs): Read storage values at a specific block.
* [example_traverse_events](/src/)
* [example_traverse_events](/src/examples/example_traverse_events.rs): Traverse events.
* [example_generic_event_callback](/src/examples/example_generic_event_callback.rs): Subsribe and wait for an event.
* [example_generic_extrinsic](/src/examples/example_generic_extrinsic.rs): Compose an extrinsic for any call in any module by supplying the module and call name as strings.
* [example_listen_new_blocks](/src/examples/example_listen_new_blocks.rs): Listen to new blocks and print them.
* [example_transfer](/src/examples/example_transfer.rs): Transfer tokens by using a wrapper of compose_extrinsic.
* [example_sudo](/src/examples/example_sudo.rs): Do a sudo call to set balance.
* [example_event_error_details](/src/examples/example_event_error_details.rs): Wait for an event and handle the event error.
