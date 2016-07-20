// Copyright 2016 Martin Grabmueller. See the LICENSE file at the
// top-level directory of this distribution for license information.
//

//! This is an experimental implementation of the Raft consensus
//! protocol [1], mainly done for educational purposes.
//!
//! [1] In Search of an Understandable Consensus Algorithm (Extended
//! Version) by Diego Ongaro and John Ousterhout,
//! https://raft.github.io/raft.pdf

extern crate uuid;

pub mod error;
pub mod config;
pub mod state;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
