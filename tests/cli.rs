use assert_cmd::Command;
use assert_fs::TempDir;
use defer::defer;
use predicates::prelude::*;

#[derive(Debug)]
struct TestData<'a> {
    cmd: &'a str,
    flag_args: Option<&'a str>,
    file_args: &'a str,
    stdout: Option<&'a str>,
    stderr: Option<&'a str>,
}

#[test]
fn cli_subcommands() {
    // setup temp directory
    let binding = TempDir::new().unwrap();
    let test_dir = binding.to_path_buf();
    defer!(binding.close().unwrap());

    let tests = &[
        // failures
        TestData {
            cmd: "create",
            flag_args: Some("-t"),
            file_args: "myfile.txt",
            stdout: None,
            stderr: Some("required arguments"),
        },
        TestData {
            cmd: "copy",
            flag_args: None,
            file_args: "",
            stdout: None,
            stderr: Some("required arguments"),
        },
        TestData {
            cmd: "copy",
            flag_args: None,
            file_args: "srcfile.txt",
            stdout: None,
            stderr: Some("required arguments"),
        },
        TestData {
            cmd: "copy",
            flag_args: Some("-x"),
            file_args: "srcfile.txt",
            stdout: None,
            stderr: Some("unexpected argument"),
        },
        TestData {
            cmd: "cat",
            flag_args: None,
            file_args: "x.txt y.txt",
            stdout: None,
            stderr: Some("required arguments"),
        },
        TestData {
            cmd: "cat",
            flag_args: None,
            file_args: "x.txt",
            stdout: None,
            stderr: Some("required arguments"),
        },
        TestData {
            cmd: "cat",
            flag_args: None,
            file_args: "",
            stdout: None,
            stderr: Some("required arguments"),
        },
        TestData {
            cmd: "del",
            flag_args: None,
            file_args: "",
            stdout: None,
            stderr: Some("required arguments"),
        },
        TestData {
            cmd: "copy",
            flag_args: None,
            file_args: "srcfile.txt -t",
            stdout: None,
            stderr: Some("No such file"),
        },
        TestData {
            cmd: "copy",
            flag_args: None,
            file_args: "srcfile.txt -t myfile.txt",
            stdout: None,
            stderr: Some("unexpected argument"),
        },
        TestData {
            cmd: "",
            flag_args: None,
            file_args: "",
            stdout: None,
            stderr: Some("unrecognized subcommand"),
        },
        // successes
        TestData {
            cmd: "create",
            flag_args: None,
            file_args: "out1.txt",
            stdout: Some("Created"),
            stderr: None,
        },
        TestData {
            cmd: "create",
            flag_args: Some("-t \'this is some text\'"),
            file_args: "out2.txt",
            stdout: Some("Created"),
            stderr: None,
        },
        TestData {
            cmd: "copy",
            flag_args: None,
            file_args: "out2.txt out3.txt",
            stdout: Some("Copied"),
            stderr: None,
        },
        TestData {
            cmd: "cat",
            flag_args: None,
            file_args: "out2.txt out3.txt out4.txt",
            stdout: Some("Concatenated"),
            stderr: None,
        },
        TestData {
            cmd: "del",
            flag_args: None,
            file_args: "out4.txt",
            stdout: Some("Deleted"),
            stderr: None,
        },
    ];

    for test in tests.iter() {
        let mut args = vec![test.cmd.to_string()];

        if test.flag_args.is_some() {
            let fa = test.flag_args.unwrap();
            if fa.contains(" ") {
                let flags = test.flag_args.unwrap().split_once(" ").unwrap();
                let mut flag_vec = vec![flags.0.to_string(), flags.1.to_string()];
                args.append(&mut flag_vec);
            } else {
                args.append(&mut vec![fa.to_string()]);
            }
        }

        let mut files: Vec<String> = test
            .file_args
            .split_whitespace()
            .map(|file| test_dir.join(file).to_str().unwrap().to_string())
            .collect();
        args.append(&mut files);

        if let Some(s) = test.stdout {
            Command::cargo_bin("filey")
                .unwrap()
                .args(args.clone())
                .assert()
                .stdout(predicate::str::contains(s))
                .success();
            ()
        };

        if let Some(s) = test.stderr {
            Command::cargo_bin("filey")
                .unwrap()
                .args(args)
                .assert()
                .stderr(predicate::str::contains(s))
                .failure();
            ()
        };
    }
}
