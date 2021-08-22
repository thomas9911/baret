use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn config_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("-c").arg("tests/test_data/doesnt/exist.yaml");
    cmd.assert().failure();

    Ok(())
}

#[test]
fn prints_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    let output = cmd.arg("--example").output()?;

    let data: serde_yaml::Value = serde_yaml::from_slice(&output.stdout)?;

    assert!(data.get("setup").is_some());
    assert!(data.get("test").is_some());
    assert!(data.get("global").is_some());
    assert!(data.get("notexisting").is_none());

    Ok(())
}

#[test]
fn run_simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("-c").arg("tests/test_data/simple.yaml");
    cmd.assert().success();

    Ok(())
}

#[test]
fn verify_simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("--verify")
        .arg("-c")
        .arg("tests/test_data/simple.yaml");
    cmd.assert().success();

    Ok(())
}

#[test]
fn run_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("-c").arg("tests/test_data/example.yaml");
    cmd.assert().success();

    Ok(())
}

#[test]
fn verify_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("--verify")
        .arg("-c")
        .arg("tests/test_data/example.yaml");
    cmd.assert().success();

    Ok(())
}

#[test]
fn run_more() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("-c").arg("tests/test_data/more.yaml");
    cmd.assert().success();

    Ok(())
}

#[test]
fn verify_more() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("--verify")
        .arg("-c")
        .arg("tests/test_data/more.yaml");
    cmd.assert().success();

    Ok(())
}

#[test]
fn run_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("-c").arg("tests/test_data/error.yaml");
    let assertion = cmd.assert().failure();

    assertion
        .stderr(predicate::str::contains(
            r#"Failed test: 'breaks'
exit code: 1
stdout:
im now going to break :D

stderr:
"#,
        ))
        .stderr(predicate::str::contains(
            r#"Failed test: 'another'
exit code: 1
stdout:
im now going to again :D

stderr:
"#,
        ))
        .stderr(predicate::str::contains("Error: Some tests had errors"));

    Ok(())
}

#[test]
fn verify_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("--verify")
        .arg("-c")
        .arg("tests/test_data/error.yaml");
    cmd.assert().success();

    Ok(())
}

#[test]
fn run_env_check() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("-c")
        .env("TESTING_FROM_TEST", "enabled")
        .arg("tests/test_data/env_check.yaml");
    cmd.assert().success();

    Ok(())
}

#[test]
fn verify_env_check() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("--verify")
        .arg("-c")
        .env("TESTING_FROM_TEST", "enabled")
        .arg("tests/test_data/env_check.yaml");
    cmd.assert().success();

    Ok(())
}

#[test]
fn run_python_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("-c").arg("tests/test_data/python_command.yaml");
    let assertion = cmd.assert().failure();

    assertion
        .stderr(predicate::str::contains("Failed test: 'fails'"))
        .stderr(predicate::str::contains("Failed test: 'success'").not());
    Ok(())
}

#[test]
fn verify_python_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("--verify")
        .arg("-c")
        .arg("tests/test_data/python_command.yaml");
    cmd.assert().success();

    Ok(())
}

#[test]
fn run_meta() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("-c").arg("tests/test_data/meta.yaml");
    let assertion = cmd.assert().failure();

    // output of the cargo test
    assertion.stderr(predicate::str::contains("test meta_failure ... FAILED"));
    Ok(())
}

#[test]
fn verify_meta() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.arg("--verify")
        .arg("-c")
        .arg("tests/test_data/meta.yaml");
    cmd.assert().success();

    Ok(())
}

#[test]
#[ignore]
fn meta_failure() -> () {
    // test that fails, but is used for the meta test
    panic!()
}