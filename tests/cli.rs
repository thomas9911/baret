use assert_cmd::Command;

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
    let output = assertion.get_output();

    assert_eq!(output.status.code().unwrap(), 1);
    assert_eq!(
        String::from_utf8(output.stderr.clone()).unwrap(),
        r#"Failed test: 'breaks'
exit code: 1
stdout:
im now going to break :D

stderr:


Failed test: 'another'
exit code: 1
stdout:
im now going to again :D

stderr:


Error: Some tests had errors
"#
    );

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
