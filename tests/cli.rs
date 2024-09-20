use assert_cmd::Command;

#[test]
fn runs() {
    Command::cargo_bin("filey").unwrap().assert().success();
}
