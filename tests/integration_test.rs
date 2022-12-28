use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_verbs_command() {
    let mut cmd = Command::cargo_bin("verb_memorizer_beta").unwrap();
    cmd.arg("verbs").assert().success();

    let output_string = std::str::from_utf8(&cmd.output().unwrap().stdout)
        .unwrap()
        .to_string();

    let verbs: Vec<&str> = output_string.split('\n').collect();
    assert!(verbs.len() > 20);
}

#[test]
fn test_check_command_should_be_unsuccessful() {
    let mut cmd = Command::cargo_bin("verb_memorizer_beta").unwrap();
    cmd.arg("check")
        .arg("-v be")
        .arg("-f")
        .arg("was ben")
        .assert()
        .success();

    assert_eq!(
        "{\"is_success\":false,\"msg\":\"be - was - been\"}\n",
        std::str::from_utf8(&cmd.output().unwrap().stdout)
            .unwrap()
            .to_string()
    );
}

#[test]
fn test_check_command_should_return_error() {
    let mut cmd = Command::cargo_bin("verb_memorizer_beta").unwrap();
    cmd.arg("check")
        .arg("-v be")
        .arg("-f")
        .arg("wasbeen")
        .assert()
        .failure();
}

#[test]
fn test_check_command_should_be_successful() {
    let mut cmd = Command::cargo_bin("verb_memorizer_beta").unwrap();
    cmd.arg("check")
        .arg("-v be")
        .arg("-f")
        .arg("was been")
        .assert()
        .success();

    assert_eq!(
        "{\"is_success\":true,\"msg\":\"\"}\n",
        std::str::from_utf8(&cmd.output().unwrap().stdout)
            .unwrap()
            .to_string()
    );
}
