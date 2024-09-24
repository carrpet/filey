use anyhow::{anyhow, Ok, Result};
use std::{
    fs::{copy, remove_file, File},
    io::Write,
    os::unix::fs::FileExt,
    path::Path,
};

pub fn create_file(file_path: &Path, text: Option<&str>) -> Result<()> {
    let mut file = File::create_new(file_path)?;

    if let Some(t) = text {
        file.write_all(t.as_bytes())?;
    }

    Ok(())
}

pub fn copy_file(source: &Path, dst: &Path) -> Result<()> {
    if dst.exists() {
        return Err(anyhow!("destination file exists"));
    }

    copy(source, dst)?;
    //TODO: handle the case where src and dst is the same when we allow overwriting

    Ok(())
}

pub fn cat_files(file1: &str, file2: &str, dst: &str) -> Result<()> {
    let buf1 = std::fs::read(file1)?;
    let buf2 = std::fs::read(file2)?;
    let mut file = File::create_new(dst)?;
    file.write_all(&buf1)?;
    file.write_all_at(&buf2, (buf1.len() + 1).try_into().unwrap())?;

    Ok(())
}

pub fn delete_file(filename: &str) -> Result<()> {
    remove_file(filename)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::create_dir;

    use super::*;
    use anyhow::anyhow;
    use assert_fs::fixture::ChildPath;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use defer::defer;

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

            let expected_error = format!("{}", d.result.as_ref().unwrap_err());
            let actual_error = format!("{}", actual_result.unwrap_err());
            assert!(actual_error.contains(&expected_error), "{}", msg);
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

        // The tests can now be specified as a set of inputs and outputs
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

            let expected_error = format!("{}", d.result.as_ref().unwrap_err());
            let actual_error = format!("{}", actual_result.unwrap_err());
            assert!(actual_error.contains(&expected_error), "{}", msg);
        }
    }
}
