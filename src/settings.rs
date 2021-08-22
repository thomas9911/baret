use std::collections::HashMap;

use derive_more::Deref;
use serde::{Deserialize, Serialize};

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

    pub fn clear_env(&self) -> bool {
        for layer in self.layer {
            if let Some(clear_env) = layer.clear_env {
                return clear_env;
            }
        }

        self.root.clear_env()
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

    pub fn env(&'a self) -> Box<dyn Iterator<Item = (&String, &String)> + 'a> {
        let mut iter: Box<dyn Iterator<Item = (&String, &String)>> =
            Box::new(self.root.env().into_iter());
        for layer in self.layer {
            iter = Box::new(iter.chain(layer.env()))
        }

        iter
    }

    pub fn to_settings(&self) -> Settings {
        Settings {
            timeout: Some(self.timeout()),
            setup_timeout: Some(self.setup_timeout()),
            command: Some(self.command().to_string()),
            clear_env: Some(self.clear_env()),
            env: self.env().map(|(k, v)| (k.clone(), v.clone())).collect(),
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
    /// clear the enviroment variables before executing the command, default false
    clear_env: Option<bool>,
    /// Add env
    #[serde(default)]
    env: HashMap<String, String>,
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

    pub fn clear_env(&self) -> bool {
        if let Some(clear_env) = self.clear_env {
            return clear_env;
        }

        false
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn return_defaults(&self) -> Settings {
        let mut env = HashMap::new();
        env.insert(String::from("MY_CUSTOM_VAR"), String::from("my_value"));
        env.insert(
            String::from("ANOTHER_CUSTOM_VAR"),
            String::from("other_value"),
        );
        Settings {
            timeout: Some(self.timeout()),
            setup_timeout: Some(self.setup_timeout()),
            command: Some(self.command().to_string()),
            clear_env: Some(self.clear_env()),
            env,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Deref, Serialize, Deserialize)]
pub struct GlobalSettings {
    /// Amount of test that run at the same time. You can increase this to speed up the tests if your processor can handle it. Or lower it if you computer freezes while running the tests, default 64.
    max_test_concurrency: Option<usize>,
    #[deref]
    #[serde(flatten)]
    other_settings: Settings,
}

impl GlobalSettings {
    pub fn max_test_concurrency(&self) -> usize {
        if let Some(max_test_concurrency) = self.max_test_concurrency {
            return max_test_concurrency;
        }

        64
    }

    pub fn return_defaults(&self) -> GlobalSettings {
        GlobalSettings {
            max_test_concurrency: Some(self.max_test_concurrency()),
            other_settings: self.other_settings.return_defaults(),
        }
    }
}
