use std::collections::VecDeque;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};
use anyhow::Error;
use jobsys::{JobInstance, JobScope, JobSystem};
use tokio::sync::{AcquireError, Semaphore, SemaphorePermit};
use tracing::{error, info};
use tracing::log::warn;
use crate::config::Config;
use crate::database::Database;

pub struct Task {
    func: Box<dyn FnMut() -> Result<(), Error>>,
    result: TaskResult
}

pub struct TaskResult {

}

impl Future for TaskResult {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

pub struct AppCtx {
    pub config: Config,
    pub database: Database,
    _job_system: JobSystem,
    job_scope: JobScope,
}

impl AppCtx {
    pub async fn new(config: Config) -> Result<Self, Error> {
        const CAPACITY: usize = 16384;
        info!("Create job pool of {CAPACITY} tasks for {} threads", config.backend_config.max_parallel_task);
        let job_system = JobSystem::new(config.backend_config.max_parallel_task, CAPACITY).unwrap();
        let job_scope = JobScope::new_from_system(&job_system);
        let database = Database::new(&config.backend_config).await?;

        Ok(Self {
            config,
            database,
            _job_system: job_system,
            job_scope,
        })
    }
    pub fn new_task_and_wait<Job>(&self, job: Job) -> Result<(), Error>
    where
        Job: Sized + Send + FnOnce() -> Result<(), Error>,
    {
        let result = Arc::new(RwLock::new(Err(Error::msg("Unhandled path in job task"))));

        let task_fn = || {
            *result.clone().write().unwrap() = job();
        };
        let job = match JobInstance::create(&self.job_scope, task_fn) {
            Ok(res) => { res }
            Err(err) => { return Err(Error::msg(format!("{:?}", err))); }
        };
        match job.wait() {
            Ok(_) => {}
            Err(err) => { return Err(Error::msg(format!("{:?}", err))); }
        };
        let result_value = result.read().unwrap();
        match result_value.as_ref() {
            Ok(_) => { Ok(()) }
            Err(err) => { Err(Error::msg(format!("Task failed : {err}"))) }
        }
    }

    pub fn new_task<Job>(&self, job: Job) -> Result<JobInstance<Job>, Error>
    where
        Job: Sized + Send + FnOnce(),
    {
        Ok(match JobInstance::create(&self.job_scope, job) {
            Ok(res) => { res }
            Err(err) => { return Err(Error::msg(format!("{:?}", err))); }
        })
    }

    pub fn app_job_scope(&self) -> &JobScope {
        &self.job_scope
    }
    pub fn jobsys(&self) -> &JobSystem {
        &self._job_system
    }

}