use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::fs::read_to_string;
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

#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// struct for holding the actual test case
pub struct Group {
    /// script to run before the test
    pub before: Option<String>,
    /// script to run after the test
    pub after: Option<String>,
    /// the test script file regexes
    #[serde_as(as = "serde_with::OneOrMany<serde_with::DisplayFromStr>")]
    pub files: Vec<glob::Pattern>,

    #[serde(default, flatten)]
    pub settings: Settings,
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        (self.before == other.before)
            && (self.after == other.after)
            && (self.settings == other.settings)
            && (self.files.len() == other.files.len())
            && self
                .files
                .iter()
                .zip(other.files.iter())
                .all(|(a, b)| a.as_str() == b.as_str())
    }
}

impl Group {
    pub fn into_tests(self) -> Result<Tests, Error> {
        let mut hashmap = HashMap::new();
        for item in self.files()? {
            let path = item?;

            hashmap.insert(
                path.to_string_lossy().to_string(),
                Test {
                    after: self.before.clone(),
                    before: self.before.clone(),
                    settings: self.settings.clone(),
                    test: read_to_string(&path)?,
                },
            );
        }

        Ok(hashmap)
    }

    pub fn files(&self) -> Result<Box<dyn Iterator<Item = glob::GlobResult>>, Error> {
        let mut iterator: Box<dyn Iterator<Item = _>> = Box::new(std::iter::Empty::default());
        for file in self.files.clone() {
            let paths = glob::glob(file.as_str())?;
            iterator = Box::new(iterator.chain(paths));
        }

        Ok(iterator)
    }
}
