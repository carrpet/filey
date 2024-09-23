use anyhow::{Ok, Result};
use std::{fs::{copy, remove_file, File}, io::Write, os::unix::fs::FileExt, path::Path};

pub fn create_file(file_path: &Path, text: Option<&str>) -> Result<()> {
    let mut file = File::create_new(file_path)?;
 
    if let Some(t) = text {
        file.write_all(t.as_bytes())?;
    } 

    Ok(())

}

pub fn copy_file(source: &str, dst: &str) -> Result<()> {
    //TODO: handle the case where src and dst is the same when we allow overwriting
    copy(source, dst)?;
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
    use super::*;
    use assert_fs::fixture::ChildPath;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use defer::defer;
    use anyhow::anyhow;
    

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

        // The tests can now be specified as a set of inputs and outputs
        let tests = &[
            // Failure scenarios
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
            }
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


}