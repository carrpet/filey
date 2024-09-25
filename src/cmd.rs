use anyhow::{anyhow, Ok, Result};
use std::{
    fs::{copy, remove_file, File},
    io::Write,
    os::unix::fs::FileExt,
    path::Path,
};

pub fn create_file(file_path: &Path, text: Option<&str>) -> Result<String> {
    let mut file = File::create_new(file_path)?;

    if let Some(t) = text {
        file.write_all(t.as_bytes())?;
    }

    let msg = format!("Created file successfully: {}", file_path.to_str().unwrap_or_default());

    Ok(msg)
}

pub fn copy_file(source: &Path, dst: &Path) -> Result<String> {
    if dst.exists() {
        return Err(anyhow!("destination file exists"));
    }

    copy(source, dst)?;
    
    let msg = format!("Copied file successfully: {} {}", 
        source.to_str().unwrap_or_default(), 
        dst.to_str().unwrap_or_default()
    );
   
    Ok(msg)
}

pub fn cat_files(file1: &Path, file2: &Path, dst: &Path) -> Result<String> {
    let buf1 = std::fs::read(file1)?;
    let buf2 = std::fs::read(file2)?;
    let mut file = File::create_new(dst)?;
    file.write_all(&buf1)?;
    file.write_all_at(&buf2, (buf1.len()).try_into().unwrap())?;

    let msg = format!("Concatenated files successfully: {} {} {}", 
        file1.to_str().unwrap_or_default(), 
        file2.to_str().unwrap_or_default(), 
        dst.to_str().unwrap_or_default()
    );

    Ok(msg)
}

pub fn delete_file(filename: &Path) -> Result<String> {
    remove_file(filename)?;

    let msg = format!("Deleted file successfully: {}", filename.to_str().unwrap_or_default());

    Ok(msg)
}

#[cfg(test)]
mod tests {
    use std::fs::create_dir;
    use std::panic;

    use super::*;
    use anyhow::anyhow;
    use assert_fs::fixture::ChildPath;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use defer::defer;

    // test utilities
    fn verify_result(actual_result: Result<String>, expected_result: &Result<()>, msg: String) {
        // Verify that we should receive an error
        let actual_error = format!("{}", actual_result.unwrap_err());
        assert!(
            expected_result.as_ref().is_err(),
            "{} {}",
            actual_error,
            msg
        );

        // Verify error message
        let expected_error = format!("{}", expected_result.as_ref().unwrap_err());
        assert!(actual_error.contains(&expected_error), "{}", msg);
    }

    #[test]
    fn test_create_file() {
        #[derive(Debug)]
        struct TestData<'a> {
            path: &'a Path,
            text: Option<&'a str>,
            result: Result<()>,
        }

        let binding = TempDir::new().unwrap();
        let test_dir = binding.to_path_buf();
        defer!(binding.close().unwrap());

        let tests = &[
            TestData {
                path: &ChildPath::new(test_dir.clone()).child("test.txt"),
                text: None,
                result: Ok(()),
            },
            TestData {
                path: &ChildPath::new(test_dir.clone()).child("test.txt"),
                text: None,
                result: Err(anyhow!("File exists")),
            },
            TestData {
                path: &ChildPath::new(test_dir).child("test_text.txt"),
                text: Some("Lorem ipsum dolor sit amet, consectetur adipiscing elit."),
                result: Ok(()),
            },
        ];

        // Run the tests
        for (i, d) in tests.iter().enumerate() {
            let msg = format!("test[{}]: {:?}", i, d);
            let actual_result = create_file(d.path, d.text);
            let msg = format!("{}, result: {:?}", msg, actual_result);

            if actual_result.is_ok() {
                let cp = ChildPath::new(d.path);
                cp.assert(predicates::path::exists());
                cp.assert(d.text.unwrap_or(""));
                continue;
            }

            verify_result(actual_result, &d.result, msg);
        }
    }

    #[test]
    fn test_copy_file() {
        #[derive(Debug)]
        struct TestData<'a> {
            src: &'a Path,
            dst: &'a Path,
            result: Result<()>,
        }

        let binding = TempDir::new().unwrap();
        let test_dir = binding.to_path_buf();
        defer!(binding.close().unwrap());

        // setup
        let test_file = "test.txt";
        let mut created = File::create(test_dir.clone().join(test_file)).unwrap();
        let unexpected_dir = "unexpected_dir";
        create_dir(test_dir.clone().join(unexpected_dir)).unwrap();
        let contents = "Aliquam ac quam ante. Curabitur a malesuada est. Integer in porta magna. Donec porta, nisl non venenatis ullamcorper, metus ligula molestie velit, in tristique diam tortor eu velit. Morbi elementum nunc vel ante tincidunt luctus. Suspendisse ornare semper nibh, ac egestas velit posuere ac. Praesent dictum metus ut cursus euismod.";
        created.write_all(contents.as_bytes()).unwrap();

        let tests = &[
            // Failures
            TestData {
                src: &ChildPath::new(test_dir.clone()).child(test_file),
                dst: &ChildPath::new(test_dir.clone()).child(test_file),
                result: Err(anyhow!("destination file exists")),
            },
            TestData {
                src: &ChildPath::new(test_dir.clone()).child("nonexistent.txt"),
                dst: &ChildPath::new(test_dir.clone()).child("dst.txt"),
                result: Err(anyhow!("No such file")),
            },
            TestData {
                src: &ChildPath::new(test_dir.clone()).child("nonexistent.txt"),
                dst: &ChildPath::new(test_dir.clone()).child("dst.txt"),
                result: Err(anyhow!("No such file")),
            },
            TestData {
                src: &ChildPath::new(test_dir.clone()).child(unexpected_dir),
                dst: &ChildPath::new(test_dir.clone()).child("dst.txt"),
                result: Err(anyhow!("source path is neither a regular file")),
            },
            TestData {
                src: &ChildPath::new(test_dir.clone()).child(test_file),
                dst: &ChildPath::new(test_dir.clone()).child(unexpected_dir),
                result: Err(anyhow!("destination file exists")),
            },
            // Successes
            TestData {
                src: &ChildPath::new(test_dir.clone()).child(test_file),
                dst: &ChildPath::new(test_dir).child("dst.txt"),
                result: Ok(()),
            },
        ];

        // Run the tests
        for (i, d) in tests.iter().enumerate() {
            let msg = format!("test[{}]: {:?}", i, d);
            let actual_result = copy_file(d.src, d.dst);
            let msg = format!("{}, result: {:?}", msg, actual_result);

            if actual_result.is_ok() {
                let cp = ChildPath::new(d.dst);
                cp.assert(predicates::path::exists());
                cp.assert(contents);
                continue;
            }

            verify_result(actual_result, &d.result, msg);
        }
    }

    #[test]
    fn test_cat_file() {
        #[derive(Debug)]
        struct TestData<'a> {
            file1: &'a Path,
            file2: &'a Path,
            dst: &'a Path,
            contents: Option<&'a str>,
            result: Result<()>,
        }

        let binding = TempDir::new().unwrap();
        let test_dir = binding.to_path_buf();
        defer!(binding.close().unwrap());

        // setup test data
        let src1 = "src1.txt";
        let src2 = "src2.txt";
        let mut created = File::create(test_dir.clone().join(src1)).unwrap();
        let mut created2 = File::create(test_dir.clone().join(src2)).unwrap();
        let unexpected_dir = "unexpected_dir";
        create_dir(test_dir.clone().join(unexpected_dir)).unwrap();
        let contents = "Maecenas pharetra efficitur nunc, vel laoreet mauris ullamcorper eget. Ut sed venenatis augue.";
        let contents2 = "Vivamus commodo pharetra diam at ornare. Vestibulum ut pharetra.";
        created.write_all(contents.as_bytes()).unwrap();
        created2.write_all(contents2.as_bytes()).unwrap();

        let same_file_result = contents.to_owned() + contents;
        let cat_files_result = contents.to_owned() + contents2;

        // test table
        let tests = &[
            // failures
            TestData {
                file1: &ChildPath::new(test_dir.clone()).child(src1),
                file2: &ChildPath::new(test_dir.clone()).child(src1),
                dst: &ChildPath::new(test_dir.clone()).child(src2),
                result: Err(anyhow!("File exists")),
                contents: None,
            },
            TestData {
                file1: &ChildPath::new(test_dir.clone()).child("nonexistent1.txt"),
                file2: &ChildPath::new(test_dir.clone()).child(src1),
                dst: &ChildPath::new(test_dir.clone()).child("dst .txt"),
                result: Err(anyhow!("No such file")),
                contents: None,
            },
            TestData {
                file1: &ChildPath::new(test_dir.clone()).child("nonexistent1.txt"),
                file2: &ChildPath::new(test_dir.clone()).child("nonexistent2.txt"),
                dst: &ChildPath::new(test_dir.clone()).child("dst.txt"),
                result: Err(anyhow!("No such file")),
                contents: None,
            },
            TestData {
                file1: &ChildPath::new(test_dir.clone()).child(src1),
                file2: &ChildPath::new(test_dir.clone()).child("nonexistent2.txt"),
                dst: &ChildPath::new(test_dir.clone()).child("dst.txt"),
                result: Err(anyhow!("No such file")),
                contents: None,
            },
            TestData {
                file1: &ChildPath::new(test_dir.clone()).child(unexpected_dir),
                file2: &ChildPath::new(test_dir.clone()).child(src1),
                dst: &ChildPath::new(test_dir.clone()).child("dst.txt"),
                result: Err(anyhow!("Is a directory")),
                contents: None,
            },
            TestData {
                file1: &ChildPath::new(test_dir.clone()).child(src1),
                file2: &ChildPath::new(test_dir.clone()).child(unexpected_dir),
                dst: &ChildPath::new(test_dir.clone()).child("dst.txt"),
                result: Err(anyhow!("Is a directory")),
                contents: None,
            },
            TestData {
                file1: &ChildPath::new(test_dir.clone()).child(unexpected_dir),
                file2: &ChildPath::new(test_dir.clone()).child(unexpected_dir),
                dst: &ChildPath::new(test_dir.clone()).child("dst.txt"),
                result: Err(anyhow!("Is a directory")),
                contents: None,
            },
            // successes
            TestData {
                file1: &ChildPath::new(test_dir.clone()).child(src1),
                file2: &ChildPath::new(test_dir.clone()).child(src1),
                dst: &ChildPath::new(test_dir.clone()).child("dst.txt"),
                result: Ok(()),
                contents: Some(&same_file_result),
            },
            TestData {
                file1: &ChildPath::new(test_dir.clone()).child(src1),
                file2: &ChildPath::new(test_dir.clone()).child(src2),
                dst: &ChildPath::new(test_dir.clone()).child("dst2.txt"),
                result: Ok(()),
                contents: Some(&cat_files_result),
            },
        ];

        // Run the tests
        for (i, d) in tests.iter().enumerate() {
            let msg = format!("test[{}]: {:?}", i, d);
            let actual_result = cat_files(d.file1, d.file2, d.dst);
            let msg = format!("{}, result: {:?}", msg, actual_result);

            if actual_result.is_ok() {
                let cp = ChildPath::new(d.dst);
                let exists = panic::catch_unwind(|| cp.assert(predicates::path::exists()));
                if exists.is_err() {
                    panic!("{}", msg);
                }
                let verify_contents = panic::catch_unwind(|| cp.assert(d.contents.unwrap()));
                if verify_contents.is_err() {
                    panic!("{}", msg);
                }
                continue;
            }

            verify_result(actual_result, &d.result, msg);
        }
    }

    #[test]
    fn test_delete_file() {
        #[derive(Debug)]
        struct TestData<'a> {
            path: &'a Path,
            result: Result<()>,
        }

        //setup temp directory
        let binding = TempDir::new().unwrap();
        let test_dir = binding.to_path_buf();
        defer!(binding.close().unwrap());

        // setup test data
        let src = "src.txt";
        File::create(test_dir.clone().join(src)).unwrap();
        let unexpected_dir = "unexpected_dir";
        create_dir(test_dir.clone().join(unexpected_dir)).unwrap();

        let tests = &[
            // failures
            TestData {
                path: &ChildPath::new(test_dir.clone()).child("nonexistent.txt"),
                result: Err(anyhow!("No such file")),
            },
            TestData {
                path: &ChildPath::new(test_dir.clone()).child(unexpected_dir),
                result: Err(anyhow!("directory")),
            },
            // success
            TestData {
                path: &ChildPath::new(test_dir).child(src),
                result: Ok(()),
            },
        ];

        // Run the tests
        for (i, d) in tests.iter().enumerate() {
            let msg = format!("test[{}]: {:?}", i, d);
            let actual_result = delete_file(d.path);
            let msg = format!("{}, result: {:?}", msg, actual_result);

            if actual_result.is_ok() {
                let cp = ChildPath::new(d.path);
                cp.assert(predicates::path::missing());
                continue;
            }

            verify_result(actual_result, &d.result, msg);
        }
    }
}
