use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::hooks::HookResult;
use crate::task::TaskType;
use crate::{hooks::Hooks, scheduler::Scheduler};
use anyhow::Result;
use async_trait::async_trait;
use tokio::time::sleep;

#[tokio::test]
async fn test_scheduler() -> Result<()> {
    let sched = Scheduler::builder().build();
    sched
        .run_task("task", async {
            sleep(Duration::from_millis(0)).await;
            println!("task");
        })
        .await?;
    sched.wait().await?;
    Ok(())
}

#[tokio::test]
async fn test_scheduler_hooks() -> Result<()> {
    let hooks = TestHooks::new();
    let sched = Scheduler::builder().hooks(hooks.clone()).build();
    for _ in 0..10 {
        sched.run_task("task", async {}).await?;
    }
    sched.wait().await?;
    assert_eq!(10, hooks.get_count());
    Ok(())
}

#[derive(Clone)]
struct TestHooks {
    count: Arc<Mutex<usize>>,
}

impl TestHooks {
    fn new() -> Self {
        TestHooks {
            count: Arc::new(Mutex::new(0)),
        }
    }
    fn get_count(&self) -> usize {
        let count = self.count.lock().unwrap();
        *count
    }
}

#[async_trait]
impl Hooks for TestHooks {
    async fn on_task_start(&mut self, typ: &TaskType) -> HookResult {
        println!("Hook: on_task_start: {:?}", typ);
        let mut count = self.count.lock().unwrap();
        *count = *count + 1;
        Ok(())
    }
}
