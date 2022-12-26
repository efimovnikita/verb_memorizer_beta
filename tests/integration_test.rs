use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_verbs_command() {
    let mut cmd = Command::cargo_bin("verb_memorizer_beta").unwrap();
    cmd.arg("verbs").assert().success().stdout(
        "be\nhave\ndo\nsay\ngo\nget\nmake\nknow\nthink\ntake\nsee\ncome\nwant\ngive\nuse\nfind\ntell\nask\nwork\nseem\nfeel\ntry\nleave\ncall\npay\nshow\nbecome\nput\nmean\nkeep\nlet\nbegin\nseem\nhelp\noffer\nplay\nrun\nmove\nlike\nlive\nbelieve\nhold\nbring\nhappen\nwrite\nsit\nstand\nlose\nspeak\nread\n", );
}
