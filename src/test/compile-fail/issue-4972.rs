// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


trait MyTrait { }

pub enum TraitWrapper {
    A(Box<MyTrait+'static>),
}

fn get_tw_map(tw: &TraitWrapper) -> &MyTrait {
    match *tw {
        TraitWrapper::A(box ref map) => map, //~ ERROR cannot be dereferenced
    }
}

pub fn main() {}
