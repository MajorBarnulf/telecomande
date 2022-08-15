//! # telecomande
//!
//! A small crate providing a primitive for the execution of asynchronous tasks by managers through signals.
//!
//! ## Example:
//! ```
//! #[tokio::main]
//! async fn main() {
//!     #[derive(Debug)]
//!     pub enum Signal {
//!         Greet,
//!         Say(String),
//!     }
//!
//!     pub struct Mgr {
//!         greeting: String,
//!     }
//!     #[telecomande::async_trait]
//!     impl telecomande::Manager for Mgr {
//!         type Signal = Signal;
//!         async fn handle(&mut self, signal: Self::Signal) {
//!             match signal {
//!                 Signal::Greet => println!("{}", self.greeting),
//!                 Signal::Say(text) => println!("{text}"),
//!             }
//!         }
//!     }
//!
//!     let manager = Mgr {
//!         greeting: "Hello".into(),
//!     };
//!     let handle = telecomande::spawn(manager);
//!
//!     let remote = handle.remote();
//!     tokio::spawn(async move {
//!         remote.send(Signal::Greet).unwrap();
//!         remote.send(Signal::Say("telecomande".into())).unwrap();
//!     })
//!     .await
//!     .unwrap();
//!
//!     //   out:
//!     // Hello
//!     // telecomande
//! }
//! ```

mod executor;
mod handle;
mod remote;
mod traits;

pub use async_trait::async_trait;
pub use executor::simple::SimpleExecutor;
pub use handle::Handle;
pub use remote::Remote;
pub use traits::{Command, Error, Executor, Processor};

#[cfg(test)]
mod examples;
