// Copyright 2017 Sean Gillespie.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module provides the transposition table implementation
//! used during search to remember previously searched positions.
use lazy_static;
use parking_lot::RwLock;
use apollo_engine::{Move, Position};
use std::collections::HashMap;

lazy_static! {
    static ref T_TABLE : RwLock<HashMap<u64, Entry>> = RwLock::new(HashMap::new());
}

#[derive(Copy, Clone, Debug)]
pub enum NodeType {
    /// A PV node is a node whose score ends up being within the alpha-beta
    /// window provided when searching. Its score is exact.
    PV(f64),

    /// An All node is a node whose score failed-high, i.e. a beta cutoff
    /// occured. Its score is not exact and is a lower bound for the exact
    /// score of this position.
    All(f64),

    /// A Cut node is a node whose score failed-low, i.e. an alpha cutoff
    /// occured. Its score is not exact and is an upper bound for the exact
    /// score of this position.
    Cut(f64)
}

#[derive(Clone, Debug)]
pub struct Entry {
    /// The best move observed for this position. May be the actual best move
    /// or a move good enough to refute the opponent's move (in the case of
    /// alpha cutoff)
    pub best_move: Move,

    /// The depth to which this position was searched.
    pub depth: u64,

    /// The type of this node when it was searched.
    pub ty: NodeType
}

/// Inserts a position into the transposition table.
pub fn insert(position: &Position, entry: Entry) {
    let hash = position.hash();
    {
        let read = T_TABLE.read();
        if read.contains_key(&hash) {
            // we've already seen this t-table.
            // TODO: consider aging-out old table entries
            return;
        }
    }

    let mut write = T_TABLE.write();
    // some other writer could have inserted it - only insert if
    // there's no key
    write.entry(hash).or_insert(entry);
}

/// Queries the transposition table for information on a given position.
/// The transposition table may have collisions and so it is not guaranteed
/// that the entry given was generated by the given position, but it
/// is unlikely.
pub fn query(position: &Position) -> Option<Entry> {
    let read = T_TABLE.read();

    // we could also have an API for this operation that calls a callback
    // with a reference to the entry. While this avoids copying the entry,
    // this results in arbitrary closures running while holding the t-table
    // read lock, which is not great.
    read.get(&position.hash()).cloned()
}

pub fn clear() {
    let mut write = T_TABLE.write();
    write.clear();
}

pub fn initialize() {
    lazy_static::initialize(&T_TABLE);
}