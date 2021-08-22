use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod command;
pub mod error;
pub mod settings;
pub mod tests;

pub use error::Error;
pub use settings::{GlobalSettings, Settings, SettingsStack};
pub use tests::{Test, Tests};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
/// Struct for holding the input test data
pub struct Data {
    #[serde(default)]
    pub setup: Setup,
    pub test: Tests,
    #[serde(default)]
    pub global: GlobalSettings,
}

impl Data {
    pub fn dump_example() -> Data {
        let mut example_test = HashMap::new();
        example_test.insert(String::from("just echo"), Test::dump_example());

        let mut base = Data::default();
        base.setup = Setup::dump_example();
        base.test = example_test;
        base.global = GlobalSettings::default().return_defaults();

        base
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
