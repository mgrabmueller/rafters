// Copyright 2016 Martin Grabmueller. See the LICENSE file at the
// top-level directory of this distribution for license information.
//

//! This module defines a data type for a Raft server configuration
//! and matching utilities.

use uuid::Uuid;

/// Configuration of a Raft server.
pub struct Config {
    /// Identifier of the Raft server.  This must be unique for the
    /// cluster, which is expected when using a properly generated
    /// UUID.
    pub this_id: Uuid,
    /// Directory to store the persistent state.  This directory must
    /// provide enough space to hold the log as well as possible
    /// snapshots and auxiliary persistent data (which is expected to
    /// be small compared to the others).
    pub state_directory: String,
}
