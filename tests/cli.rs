use assert_cmd::Command;

#[test]
fn subcommand_exit_status_success() {
    let tests = &[
        "create output.txt",
        "create -t myfile.txt output.txt",
        "copy srcfile.txt dstfile.txt",
        "cat x.txt y.txt output.txt",
        "del x.txt",
    ];

    for test in tests.iter() {
        let args: Vec<&str> = test.split_whitespace().collect();

        Command::cargo_bin("filey")
            .unwrap()
            .args(args)
            .assert()
            .success();
    }
}

#[test]
fn subcommand_exit_status_failure() {
    let tests = &[
        "create",
        "create -t myfile.txt",
        "copy",
        "copy srcfile.txt",
        "copy -x",
        "cat",
        "cat x.txt",
        "cat x.txt y.txt",
        "del",
    ];

    for test in tests.iter() {
        let args: Vec<&str> = test.split_whitespace().collect();

        Command::cargo_bin("filey")
            .unwrap()
            .args(args)
            .assert()
            .failure();
    }
}
