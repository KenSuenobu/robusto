// Robusto
// Robusto Job Dispatcher and Main Library
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

use crate::job::{Job, JobStatus};
use std::thread;

struct JobStore {
    job: Box<dyn Job>,
    status: JobStatus,
}

impl JobStore {
    pub fn new(job: Box<dyn Job>) -> Self {
        Self {
            job,
            status: JobStatus::Queued,
        }
    }
}

#[derive(Default)]
pub struct Robusto {
    jobs_list: Vec<JobStore>,
}

impl Robusto {
    pub fn add_job(&mut self, job: Box<dyn Job>) {
        self.jobs_list.push(JobStore::new(job));
    }

    pub fn run(self) {
        let mut handles = vec![];

        for mut job_store in self.jobs_list {
            handles.push(thread::spawn(move || {
                job_store.status = JobStatus::Waiting;

                // Query etcd to see if required job IDs are done running.
                job_store.status = JobStatus::Running;

                // Run the job
                let _result = job_store.job.run();

                // Check the status of the job to see if it returns a value.
                job_store.status = JobStatus::Finished;
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
