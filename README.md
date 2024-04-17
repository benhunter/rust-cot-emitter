# rust-cot-emitter

Cursor-On-Target emitter in Rust.

# Local Development with OpenTAKServer

1. Run RabbitMQ: `docker run -d --hostname rabbit-tak --name rabbit-tak -p 5672:5672 rabbitmq:3`
1. Install pipx
1. Install poetry
1. `poetry shell`
1. `poetry install`
1. Run OpenTAKServer: `TZ="US/Central" python opentakserver/app.py`

# Links

- [OpenTAKServer](https://github.com/brian7704/opentakserver)
- [OpenTAKServer Docs](https://docs.opentakserver.io/)
- [RabbitMQ DockerHub](https://hub.docker.com/_/rabbitmq)
- [Cursor-On-Target Message Router User's Guide, MITRE | PDF](https://www.mitre.org/sites/default/files/pdf/09_4937.pdf)
- [cot_publisher crate](https://crates.io/crates/cot_publisher)
- [Protocol Buffers](https://github.com/protocolbuffers/protobuf?tab=readme-ov-file#protobuf-compiler-installation)
- [Protocol Buffers | Arch package](https://archlinux.org/packages/extra/x86_64/protobuf/) - `protoc` is requried by `cot_publisher` crate.
