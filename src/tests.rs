use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::settings::{GlobalSettings, Settings, SettingsStack};
use crate::{command, Error};

pub type Tests = HashMap<String, Test>;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
/// struct for holding the actual test case
pub struct Test {
    /// script to run before the test
    pub before: Option<String>,
    /// script to run after the test
    pub after: Option<String>,
    /// the actual test script
    pub test: String,

    #[serde(default, flatten)]
    pub settings: Settings,
}

impl Test {
    pub fn dump_example() -> Test {
        Test {
            before: Some(String::new()),
            after: Some(String::new()),
            test: String::from("echo 'test'"),
            settings: Settings::default().return_defaults(),
        }
    }

    pub async fn run(&self, global: &GlobalSettings) -> Result<(), Error> {
        let stack = &[&self.settings];
        let settings = global.stack(stack);

        self.run_before(&settings).await?;
        let test_result = self.run_test(&settings).await;
        self.run_after(&settings).await?;

        test_result?;
        Ok(())
    }

    pub async fn run_arc_settings(self, global: Arc<GlobalSettings>) -> Result<(), Error> {
        let stack = &[&self.settings];
        let settings = global.stack(stack);

        self.run_before(&settings).await?;
        self.run_test(&settings).await?;
        self.run_after(&settings).await?;

        Ok(())
    }

    async fn run_before<'a, 'b>(&'a self, settings: &SettingsStack<'a, 'b>) -> Result<(), Error> {
        if let Some(ref before) = self.before {
            command::run(before, settings).await?;
        }

        Ok(())
    }

    async fn run_after<'a, 'b>(&'a self, settings: &SettingsStack<'a, 'b>) -> Result<(), Error> {
        if let Some(ref after) = self.after {
            command::run(after, settings).await?;
        }

        Ok(())
    }

    async fn run_test<'a, 'b>(&'a self, settings: &SettingsStack<'a, 'b>) -> Result<(), Error> {
        command::run(&self.test, settings).await?;
        Ok(())
    }
}
