use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod command;
pub mod error;
pub mod expression;
pub mod settings;
pub mod tests;

pub use error::Error;
pub use settings::{GlobalSettings, Settings, SettingsStack};
pub use tests::{Group, Test, Tests};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
/// Struct for holding the input test data
pub struct Data {
    #[serde(default)]
    pub setup: Setup,
    pub test: TestsOrGroup,
    #[serde(default)]
    pub global: GlobalSettings,
}

impl Data {
    pub fn dump_example() -> Data {
        let mut example_test = HashMap::new();
        example_test.insert(String::from("just echo"), Test::dump_example());

        let mut base = Data::default();
        base.setup = Setup::dump_example();
        base.test = TestsOrGroup::Tests(example_test);
        base.global = GlobalSettings::default().return_defaults();

        base
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestsOrGroup {
    Tests(Tests),
    Group(Group),
}

impl TestsOrGroup {
    pub fn len(&self) -> usize {
        use TestsOrGroup::*;

        match self {
            Tests(x) => x.len(),
            Group(x) => x.files().unwrap().collect::<Vec<_>>().len(),
        }
    }
}

impl Default for TestsOrGroup {
    fn default() -> TestsOrGroup {
        TestsOrGroup::Tests(Default::default())
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
/// setup config
pub struct Setup {
    before_all: Option<String>,
    after_all: Option<String>,

    #[serde(flatten)]
    setups: HashMap<String, String>,
}

impl Setup {
    pub fn dump_example() -> Setup {
        Setup {
            before_all: Some(String::new()),
            after_all: Some(String::new()),
            setups: HashMap::default(),
        }
    }
}
