use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_verbs_command() {
    let mut cmd = Command::cargo_bin("verb_memorizer_beta").unwrap();
    cmd.arg("verbs").assert().success().stdout(
        "be\nhave\ndo\nsay\ngo\nget\nmake\nknow\nthink\ntake\nsee\ncome\nwant\ngive\nuse\nfind\ntell\nask\nwork\nseem\nfeel\ntry\nleave\ncall\npay\nshow\nbecome\nput\nmean\nkeep\nlet\nbegin\nseem\nhelp\noffer\nplay\nrun\nmove\nlike\nlive\nbelieve\nhold\nbring\nhappen\nwrite\nsit\nstand\nlose\nspeak\nread\n", );
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
