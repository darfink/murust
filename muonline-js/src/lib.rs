#![feature(conservative_impl_trait)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate closet;
extern crate tap;

extern crate futures;
extern crate tokio;

extern crate muonline_packet as mupack;
extern crate muonline_packet_codec as mucodec;
extern crate muonline_protocol as protocol;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate jsonrpc_macros;
extern crate jsonrpc_core;
extern crate jsonrpc_http_server;

pub use self::builder::ServerBuilder;
pub use service::rpc;
use std::io;

mod builder;
mod service;

/// An implementation of a Join Server.
pub struct JoinServer {
  join_service: service::JoinService,
  rpc_service: service::RpcService,
}

impl JoinServer {
  /// Spawns a new Join Server using defaults.
  pub fn spawn() -> io::Result<Self> { Self::builder().spawn() }

  /// Returns a builder for the Join Server.
  pub fn builder() -> ServerBuilder { ServerBuilder::default() }

  /// Returns the URI of the RPC service.
  pub fn uri(&self) -> &str { self.rpc_service.uri() }

  /// Closes the server.
  pub fn close(self) -> io::Result<()> {
    let result = self.join_service.close();
    self.rpc_service.close();
    result
  }

  /// Will block, waiting for the server to finish.
  pub fn wait(self) -> io::Result<()> {
    let result = self.join_service.wait();
    // Explicitly close, and skip waiting for the RPC service.
    self.rpc_service.close();
    result
  }
}
