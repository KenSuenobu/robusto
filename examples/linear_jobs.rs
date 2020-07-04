// Robusto
// Distributed Job Engine
//
// Copyright 2020 Ken Suenobu
//
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

extern crate robusto;

use robusto::job::Job;
use robusto::robusto::*;

#[derive(Default)]
struct SimpleJobA {}

#[derive(Default)]
struct SimpleJobB {}

#[derive(Default)]
struct SimpleJobC {}

impl Job for SimpleJobA {
    fn depends_on(&self) -> Vec<&'static str> {
        vec![""]
    }

    fn run(&mut self) -> &'static str {
        eprintln!("Job A");
        "a"
    }
}

impl Job for SimpleJobB {
    fn depends_on(&self) -> Vec<&'static str> {
        vec!["a"]
    }

    fn run(&mut self) -> &'static str {
        eprintln!("Job B");
        "b"
    }
}

impl Job for SimpleJobC {
    fn depends_on(&self) -> Vec<&'static str> {
        vec!["b"]
    }

    fn run(&mut self) -> &'static str {
        eprintln!("Job C");
        "c"
    }
}

fn main() {
    let mut robusto = Robusto::default();

    robusto.add_job(Box::new(SimpleJobA::default()));
    robusto.add_job(Box::new(SimpleJobB::default()));
    robusto.add_job(Box::new(SimpleJobC::default()));
    robusto.run();
}
