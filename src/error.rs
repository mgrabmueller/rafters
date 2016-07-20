// Copyright 2016 Martin Grabmueller. See the LICENSE file at the
// top-level directory of this distribution for license information.
//

//! Errors for the Raft system.

use std::fmt;
use std::error;

/// Errors that may happen during the operation of Peerington.
#[derive(Debug)]
pub enum Error {
    /// Leader operation was attempted on non-leader.
    NotLeader,
    /// Other error with ad-hoc error message.
    Other(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotLeader => write!(f, "not a leader"),
            Error::Other(s) => write!(f, "{}", s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotLeader => "not a leader",
            Error::Other(s) => s,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::NotLeader => None,
            Error::Other(_) => None,
        }
    }
}
