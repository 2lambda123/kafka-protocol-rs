# Kafka-Protocol [![Build](https://github.com/tychedelia/kafka-protocol-rs/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/0x1991babe/kafka-protocol-rs/actions/workflows/build-and-test.yml) [![crates.io](https://img.shields.io/crates/v/kafka-protocol.svg)](https://crates.io/crates/kafka-protocol) [![docs.rs](https://img.shields.io/docsrs/kafka-protocol)](https://docs.rs/kafka-protocol)

Rust implementation of the [Kafka wire protocol](https://kafka.apache.org/protocol.html).

Unlike other Kafka protocol implementations, this project uses code generation to cover the entire Kafka API surface,
including different protocol versions. See [Kafka's repo](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message)
for an example of protocol schema.

## Versioning

Protocol messages are generated against the most recent stable Kafka release, currently [3.3.2](https://github.com/apache/kafka/releases/tag/3.3.2).

Although the Kafka protocol remains relatively stable and strives to be backwards compatible, new fields are occasionally
added. In order to ensure forward compatibility with the protocol, this crate marks all exported items as `#[non-exhaustive]`.
Protocol messages can be constructed using either `Default::default` or their provided [builder](https://docs.rs/derive_builder/latest/derive_builder/).

## Working with messages

Using `Default::default`:

```rust
use kafka_protocol::messages::{ApiKey, MetadataRequest, RequestHeader};
use kafka_protocol::protocol::StrBytes;

let mut header = RequestHeader::default();
header.client_id = Some(StrBytes::from_str("my-client"));
header.request_api_key = ApiKey::MetadataKey as i16;
header.request_api_version = 12;

let mut request = MetadataRequest::default();
request.topics = None;
request.allow_auto_topic_creation = true;
```

Using `kafka_protocol::protocol::Builder`:

```rust
use kafka_protocol::messages::{ApiKey, MetadataRequest, RequestHeader};
use kafka_protocol::protocol::{Builder, StrBytes};

let header = RequestHeader::builder()
    .client_id(Some(StrBytes::from_str("my-client")))
    .request_api_key(ApiKey::MetadataKey as i16)
    .request_api_version(12)
    .build();
!
let request = MetadataRequest::builder()
    .topics(None)
    .allow_auto_topic_creation(true)
    .build();
```

### Serialization

Once a message has been created, it can be serialized using `Encodable`, writing
the struct to a provided `bytes::BytesMut`. The API version for the given message
matching the version specified in the request header must be provided.

```rust
use bytes::BytesMut;
use kafka_protocol::messages::MetadataRequest;
use kafka_protocol::protocol::Encodable;

let mut bytes = BytesMut::new();
let request = MetadataRequest::default();
request.encode(&mut bytes, 12).unwrap();
```

### Deserialization

Messages can be decoded using `Decodable` and providing the matching API version from their
corresponding request.

```rust
use bytes::Bytes;
use kafka_protocol::messages::ApiVersionsRequest;
use kafka_protocol::protocol::Decodable;

let bytes: [u8; 25] = [
        0x12, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2d,
        0x6b, 0x61, 0x66, 0x6b, 0x61, 0x2d, 0x6a, 0x61,
        0x76, 0x61, 0x06, 0x32, 0x2e, 0x38, 0x2e, 0x30,
        0x00
];

let res = ApiVersionsRequest::decode(&mut Bytes::from(bytes.to_vec()), 3).unwrap();
```

### Development

Run `cargo run -p protocol_codegen` in the root path of this repo to generate/update the Rust codes via the latest Kafka
protocol schema.

Originally implemented by
[@Diggsey](https://github.com/Diggsey) in a minimal Kafka client implementation [Franz](https://github.com/Diggsey/franz)
