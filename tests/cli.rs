use std::fs::File;

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
    stderr: Option<&'a str>
}

#[test]
fn subcommand_exit_status_success() {

    // setup temp directory
    let binding = TempDir::new().unwrap();
    let test_dir = binding.to_path_buf();
    defer!(binding.close().unwrap());

    //setup files
    let test_file = "test.txt";
    let mut created = File::create(test_dir.clone().join(test_file)).unwrap();
    
    let tests = &[
        TestData{cmd:"create", flag_args: None, file_args:"out1.txt", stdout: Some("Created file successfully"), stderr: None},
        TestData{cmd:"create", flag_args: Some("-t \'this is some text\'"), file_args:"out2.txt", stdout: Some("Created file successfully"), stderr: None},
        TestData{cmd:"copy", flag_args: None, file_args:&(test_file.to_owned() +" out3.txt"), stdout: Some("Copied file successfully"), stderr: None},
        TestData{cmd:"cat", flag_args: None, file_args:&(test_file.to_owned() + " " + test_file +" out4.txt"), stdout: Some("Concatenated files"), stderr: None},
        //TestData{cmd:"del", flag_args: None, file_args:&(test_file.to_owned() +" out3.txt"), stdout: Some("Copied file successfully"), stderr: None},

    ];

    /*
    let tests = &[
        "create output.txt",
        "create -t myfile.txt output.txt",
        "copy srcfile.txt dstfile.txt",
        "cat x.txt y.txt output.txt",
        "del x.txt",
    ];
    */

    for test in tests.iter() {
        let mut args = vec![test.cmd.to_string()];
        let mut files: Vec<String> = test.file_args.split_whitespace().map(|file| test_dir.join(file).to_str().unwrap().to_string()).collect();
        args.append(&mut files);
        //let cmd = test.cmd.to_owned() + " " + test_dir.join(path)test.file_args;

        //let args: Vec<&str> = test.cmd.split_whitespace().collect();

        match test.stdout {
            Some(s) =>  Command::cargo_bin("filey")
            .unwrap()
            .args(args)
            .assert()
            .stdout(predicate::str::contains(s))
            .success(),
            None => todo!(),
        };
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
