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

use crate::job::Job;
use crate::jobstatus::JobStatus;
use crate::jobstatus::JobStatus::Waiting;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

struct JobStore {
    job: Arc<Mutex<Box<dyn Job>>>,
    status: JobStatus,
}

impl JobStore {
    pub fn new(job: Box<dyn Job>) -> Self {
        Self {
            job: Arc::new(Mutex::new(job)),
            status: Waiting,
        }
    }
}

#[derive(Default)]
pub struct Robusto {
    jobs_list: Arc<Mutex<Vec<JobStore>>>,
}

impl Robusto {
    pub fn add_job(&mut self, job: Box<dyn Job>) {
        self.jobs_list.lock().unwrap().push(JobStore::new(job));
    }

    pub fn run(&mut self) {
        // let mut handles = vec![];
        // let count = self.jobs_list.lock().unwrap().len();
        //
        // for i in 0..count {
        //     // let mut job_store = self.jobs_list.lock().unwrap().get_mut(i).unwrap();
        //     let job_list = &self.jobs_list;
        //
        //     handles.push(thread::spawn(move ||
        //         {
        //             self.jobs_list;
        //             eprintln!("Value: {}", i);
        //
        //             // let mut job = job_store.job.get_mut().unwrap();
        //             // let wait_keys = job.depends_on();
        //             //
        //             // job_store.status = JobStatus::Queued;
        //             // let job_key = job_store.job.lock().unwrap().run();
        //             // job_store.status = JobStatus::Finished;
        //             //
        //             // // Publish job key to etcd
        //         })
        //     );
        // }
        //
        // for handle in handles {
        //     handle.join();
        // }
    }
}
