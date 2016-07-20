// Copyright 2016 Martin Grabmueller. See the LICENSE file at the
// top-level directory of this distribution for license information.
//

//! This module defines a data type for a Raft server state and
//! matching utilities.

use uuid::Uuid;
use std::collections::BTreeMap;

use super::config::Config;
use super::error::Error;

/// Log entries describe commands to be executed on the distributed
/// state machine.
pub struct LogEntry {
    pub term: u64,
    pub command: String,
}

/// A server can have one of three roles: a follower is a node that
/// knows it's leader and takes commands.  A candidate is a node that
/// does not know a leader and attempts to get elected.  A leader is a
/// node that was elected by the majority of the nodes.
pub enum Role {
    /// Node following the leader.
    Follower,
    /// Node attempting to become the leader
    Candidate,
    /// Cluster leader.
    Leader(LeaderState),
}

/// Volatile state held by leaders.
pub struct LeaderState {
    /// "for each server, index of the next log entry to send to that
    /// server (initialized to leader last log index + 1)" [1]
    pub next_index: BTreeMap<Uuid, u64>,
    /// "for each server, index of highest log entry known to be
    /// replicated on server (initialized to 0, increases
    /// monotonically)" [1]
    pub match_index: BTreeMap<Uuid, u64>,
}

/// Structure describing the state of a Raft server.
pub struct State {
    /// Configuration of this server.
    pub config: Config,

    // "Persistent state on all servers:" [1]
    /// "latest state server has seen (initialized to 0 on first boot,
    /// increases monotonically)." [1]
    pub current_term: u64,
    /// "candidateId that received vote in current term (or null if
    /// none)" [1]
    pub voted_for: Option<Uuid>,
    /// "log entries; each entry contains command for state machine,
    /// and term when entry was received by leader (first index is 1)"
    /// [1]
    pub log: Vec<LogEntry>,

    // "Volatile state on all servers:"
    // [1]
    /// "index of highest log entry known to be committed (initialized
    /// to 0, increases monotonically)" [1]
    pub commit_index: u64,
    /// "index of highest log entry applied to state machine
    /// (initialized to 0, increases monotonically)" [1]
    pub last_applied: u64,

    // "Volatile state on leaders (Reinitialized after election):"
    // [1]
    /// Role of the server, including role-specific state.
    pub role: Role,
}

impl State {
    pub fn new(uuid: Uuid, state_dir: String) -> State {
        let config = Config {
            this_id: uuid,
            state_directory: state_dir,
        };
        State {
            config: config,
            current_term: 0,
            voted_for: None,
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
            role: Role::Follower,
        }
    }

    pub fn get_next_index(&mut self, follower_id: &Uuid) -> Result<u64, Error> {
        match self {
            &mut State { role: Role::Leader(ref mut leader_state), .. } => {
                let log_len = self.log.len() as u64;
                Ok(*leader_state.next_index.entry(*follower_id).or_insert_with(|| log_len + 1))
            }
            _ => Err(Error::NotLeader),
        }
    }

    pub fn get_match_index(&mut self, follower_id: &Uuid) -> Result<u64, Error> {
        match self {
            &mut State { role: Role::Leader(ref mut leader_state), .. } => {
                Ok(*leader_state.match_index.entry(*follower_id).or_insert_with(|| 0))
            }
            _ => Err(Error::NotLeader),
        }
    }
}
