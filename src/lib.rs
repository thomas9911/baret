use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub mod command;

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Data {
    #[serde(default)]
    pub setup: Setup,
    pub test: HashMap<String, Test>,
    #[serde(default)]
    pub global: Settings,
}

impl Data {
    pub fn dump_example() -> Data {
        let mut example_test = HashMap::new();
        example_test.insert(String::from("just echo"), Test::dump_example());

        let mut base = Data::default();
        base.setup = Setup::dump_example();
        base.test = example_test;
        base.global = Settings::default().return_defaults();

        base
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Test {
    before: Option<String>,
    after: Option<String>,
    test: String,

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

    pub async fn run(&self, global: &Settings) -> Result<(), command::Error> {
        let stack = &[&self.settings];
        let settings = global.stack(stack);

        self.run_before(&settings).await?;
        self.run_test(&settings).await?;
        self.run_after(&settings).await?;

        Ok(())
    }

    pub async fn run_arc_settings(self, global: Arc<Settings>) -> Result<(), command::Error> {
        let stack = &[&self.settings];
        let settings = global.stack(stack);

        self.run_before(&settings).await?;
        self.run_test(&settings).await?;
        self.run_after(&settings).await?;

        Ok(())
    }

    async fn run_before<'a, 'b>(
        &'a self,
        settings: &SettingsStack<'a, 'b>,
    ) -> Result<(), command::Error> {
        if let Some(ref before) = self.before {
            command::run(before, settings).await?;
        }

        Ok(())
    }

    async fn run_after<'a, 'b>(
        &'a self,
        settings: &SettingsStack<'a, 'b>,
    ) -> Result<(), command::Error> {
        if let Some(ref after) = self.after {
            command::run(after, settings).await?;
        }

        Ok(())
    }

    async fn run_test<'a, 'b>(
        &'a self,
        settings: &SettingsStack<'a, 'b>,
    ) -> Result<(), command::Error> {
        command::run(&self.test, settings).await?;
        Ok(())
    }
}

pub struct SettingsStack<'a, 'b> {
    root: &'a Settings,
    layer: &'b [&'b Settings],
}

impl<'a, 'b> SettingsStack<'a, 'b> {
    pub fn timeout(&self) -> u32 {
        for layer in self.layer {
            if let Some(timeout) = layer.timeout {
                return timeout;
            }
        }

        self.root.timeout()
    }

    pub fn setup_timeout(&self) -> u32 {
        for layer in self.layer {
            if let Some(timeout) = layer.setup_timeout {
                return timeout;
            }
        }

        self.root.setup_timeout()
    }

    pub fn command(&self) -> &str {
        for layer in self.layer {
            if let Some(ref command) = layer.command {
                return command;
            }
        }

        self.root.command()
    }

    pub fn command_with_args(&self) -> (String, shlex::Shlex) {
        let (program, program_args) = {
            let mut program_args = shlex::Shlex::new(self.command());
            match program_args.next() {
                None => {
                    let mut program_args = shlex::Shlex::new("sh -c");
                    let program = program_args.next().unwrap();
                    (program, program_args)
                }
                Some(program) => (program, program_args),
            }
        };

        (program, program_args)
    }

    pub fn to_settings(&self) -> Settings {
        Settings {
            timeout: Some(self.timeout()),
            setup_timeout: Some(self.setup_timeout()),
            command: Some(self.command().to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    /// timeout in miliseconds, default 5000 ms
    timeout: Option<u32>,
    /// setup timeout in miliseconds, default 5000 ms
    setup_timeout: Option<u32>,
    /// command to execute the tests with, default "sh -c"
    command: Option<String>,
}

impl Settings {
    pub fn stack<'a, 'b>(&'a self, other: &'b [&Settings]) -> SettingsStack<'a, 'b> {
        SettingsStack {
            root: self,
            layer: other,
        }
    }

    pub fn timeout(&self) -> u32 {
        if let Some(timeout) = self.timeout {
            return timeout;
        }

        5000
    }

    pub fn setup_timeout(&self) -> u32 {
        if let Some(timeout) = self.setup_timeout {
            return timeout;
        }

        5000
    }

    pub fn command(&self) -> &str {
        if let Some(ref command) = self.command {
            return command;
        }

        "sh -c"
    }

    pub fn return_defaults(&self) -> Settings {
        Settings {
            timeout: Some(self.timeout()),
            setup_timeout: Some(self.setup_timeout()),
            command: Some(self.command().to_string()),
        }
    }
}
