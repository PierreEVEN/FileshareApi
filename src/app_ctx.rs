use anyhow::Error;
use jobsys::{JobInstance, JobScope, JobSystem};
use crate::config::Config;
use crate::database::Database;

pub struct AppCtx {
    pub config: Config,
    pub database: Database,
    _job_system: JobSystem,
    job_scope: JobScope,
}

impl AppCtx {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let job_system = JobSystem::new(config.backend_config.max_parallel_task, 16384).unwrap();
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
        Job: Sized + Send + FnOnce(),
    {
        let job = match JobInstance::create(&self.job_scope, job) {
            Ok(res) => { res }
            Err(err) => { return Err(Error::msg(format!("{:?}", err))); }
        };
        match job.wait() {
            Ok(_) => {}
            Err(err) => { return Err(Error::msg(format!("{:?}", err))); }
        };
        Ok(())
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
}