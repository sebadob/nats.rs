// Copyright 2020-2022 The NATS Authors
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//! JetStream is a NATS built-in persistence layer providing
//! [Streams][crate::jetstream::stream::Stream] with *at least once*
//! and *exactly once* semantics.
//!
//! To start, create a new [Context] which is an entrypoint to `JetStream` API.
//!
//! # Examples:
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn mains() -> Result<(), async_nats::Error> {
//! use futures::StreamExt;
//! let client = async_nats::connect("localhost:4222").await?;
//! let jetstream = async_nats::jetstream::new(client);
//!
//! let stream = jetstream.get_or_create_stream(async_nats::jetstream::stream::Config {
//!     name: "events".to_string(),
//!     max_messages: 10_000,
//!     ..Default::default()
//! }).await?;
//!
//! jetstream.publish("events".to_string(), "data".into()).await?;
//!
//! let mut consumer = stream.get_or_create_consumer("consumer", async_nats::jetstream::consumer::pull::Config {
//!     durable_name: Some("consumer".to_string()),
//!     ..Default::default()
//! }).await?;
//!
//! let messages = consumer.process(50);
//! futures::pin_mut!(messages);
//! while let Some(message) = messages.next().await {
//!     println!("message receiver: {:?}", message?);
//! }
//! Ok(())
//! # }
//! ```

use crate::{Client, Message};

pub mod consumer;
pub mod context;
pub mod publish;
pub mod response;
pub mod stream;

pub use context::Context;

/// Creates a new JetStream [Context] that provides JetStream API for managming and using [Streams][crate::jetstream::stream::Stream],
/// [Consumers][crate::jetstream::consumer::Consumer], key value and object store.
///
/// # Examples:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), async_nats::Error> {
///
/// let client = async_nats::connect("localhost:4222").await?;
/// let jetstream = async_nats::jetstream::new(client);
///
/// jetstream.publish("subject".to_string(), "data".into()).await?;
/// # Ok(())
/// # }
/// ```
pub fn new(client: Client) -> Context {
    Context::new(client)
}

/// Creates a new JetStream [Context] with given JetStteam domain.
///
/// # Examples:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), async_nats::Error> {
///
/// let client = async_nats::connect("localhost:4222").await?;
/// let jetstream = async_nats::jetstream::with_domain(client, "hub");
///
/// jetstream.publish("subject".to_string(), "data".into()).await?;
/// # Ok(())
/// # }
/// ```
pub fn with_domain<T: AsRef<str>>(client: Client, domain: T) -> Context {
    context::Context::with_domain(client, domain)
}

/// Creates a new JetStream [Context] with given JetStream prefix.
/// By default it is `$JS.API`.
/// # Examples:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), async_nats::Error> {
///
/// let client = async_nats::connect("localhost:4222").await?;
/// let jetstream = async_nats::jetstream::with_prefix(client, "JS.acc@hub.API");
///
/// jetstream.publish("subject".to_string(), "data".into()).await?;
/// # Ok(())
/// # }
/// ```
pub fn with_prefix(client: Client, prefix: &str) -> Context {
    context::Context::with_prefix(client, prefix)
}

#[derive(Debug)]
pub struct JetStreamMessage {
    pub message: Message,
    pub context: Context,
}

impl std::ops::Deref for JetStreamMessage {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}