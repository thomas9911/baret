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
