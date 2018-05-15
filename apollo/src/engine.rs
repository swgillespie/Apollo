// Copyright 2017-2018 Sean Gillespie.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use attacks::AttackTable;

pub struct Engine {
    attack_table: Box<AttackTable>
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            attack_table: Box::new(AttackTable::new())
        }
    }

    pub fn attack_table(&self) -> &AttackTable {
        &self.attack_table
    }
}
