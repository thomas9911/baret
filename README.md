# Baret

Bash and Rust End-to-end Testing

Example config:

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

for more examples check the test/test_data folder.
