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

use std::io::Error;

/// This defines a `Job` that is used by Robusto.  All runnable `Job`s must implement
/// this trait.
pub trait Job: Send {
    /// Provides a list of the names of `Job`s that must be complete before this `Job`
    /// will trigger a `run()` call.  This is where the DAG can be built.  `Job`s can
    /// depend on more than one `Job` name to be triggered.
    fn depends_on(&self) -> Vec<&'static str>;

    /// Provides an entry point for a `Job` to run.  Once the `Job` is complete, it must
    /// return the name of the task that was run, so that other `Job`s that may depend on
    /// this `Job` can be triggered.  Returns an `Ok(())` result, or an error otherwise.
    fn run(&mut self) -> Result<(), Error>;

    /// Retrieves the name of this job, so that it can be sent to `etcd` to trigger other
    /// `Job`s.
    fn job_name(&self) -> &'static str;
}

pub enum JobStatus {
    Waiting,
    Queued,
    Running,
    Finished,
    Failed,
}
