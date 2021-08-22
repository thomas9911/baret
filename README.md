# Baret

Bash and Rust End-to-end Testing

"Parallel" test runner (we can discuss if it is parallel or concurrent). Plus it is a nice hat.

## Usage

Create a file called `baret.yaml` (or another name if you like) with the tests you want to run, see example below.

This file contains the tests, plus optional setup and settings.

Under `test.<your-test-name>.test` is the actual test to run. This script should return a non-zero exit code on error and a zero exit-code on success. For example:

```yaml
test:
  breaks:
    test: |-
      echo "im now going to break :'("
      exit 1
  works:
    test: |-
      echo 'this just works'
```

By default it uses the `sh -c` command to execute these scripts, but you can configure this to be a different command by setting the `global.command` or the `test.<your-test-name>.command` option.

If your test need some setup/breakdown you can specify this per test with the `test.<your-test-name>.before` or `test.<your-test-name>.after` options. Or globally for the `setup.before_all` or `setup.after_all` options. The `after` will run always even if you test fails.

then run:

```sh
baret
# or
baret -c your-file-with-tests.yaml
```

Done

### Simple example config

Simple config

```yaml
test:
  hallo:
    test: "echo 'hallo'"
```

### Default example config

These are all the available/default options:

```yaml
setup:
  before_all: ""
  after_all: ""
test:
  just echo:
    before: ""
    after: ""
    test: "echo 'test'"
    timeout: 5000
    setup_timeout: 5000
    command: sh -c
    clear_env: false
    env:
      MY_CUSTOM_VAR: my_value
      ANOTHER_CUSTOM_VAR: other_value
global:
  max_test_concurrency: 64
  timeout: 5000
  setup_timeout: 5000
  command: sh -c
  clear_env: false
  env:
    MY_CUSTOM_VAR: my_value
    ANOTHER_CUSTOM_VAR: other_value
```

for more examples check the [test/test_data](tests/test_data) folder.

## Instalation

```sh
# for now you can install with git
cargo install --git https://github.com/thomas9911/baret
```
