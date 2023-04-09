# voting-tonic

This repo follows the very nice blog article by Thosten Hans [Let's build a gRPC server and client in Rust with Tonic](https://www.thorsten-hans.com/grpc-services-in-rust-with-tonic/) with the accompanying repo [rusty-grpc](https://github.com/ThorstenHans/rusty-grpc)

## server

Exposes a gRPC service allowing consumers to vote *up* or *down* for a `word` of their choice.

## client

Consumes the gRPC service.
