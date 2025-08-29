use ignore::Match::Ignore;
use ignore::gitignore::Gitignore;
use std::env;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

trait Entry {
    fn path(&self) -> &Path;
    fn is_dir(&self) -> bool;
}

impl Entry for DirEntry {
    fn path(&self) -> &Path {
        self.path()
    }
    fn is_dir(&self) -> bool {
        self.file_type().is_dir()
    }
}

fn process<T, I, E>(entries: I, ignore: &Gitignore) -> impl Iterator<Item = Result<T, E>>
where
    T: Entry,
    I: Iterator<Item = Result<T, E>>,
{
    entries.filter_map(|e| match e {
        Ok(entry) => {
            if let Ignore(_) = ignore.matched(entry.path(), entry.is_dir()) {
                return Some(Ok(entry));
            }
            None
        }
        Err(e) => Some(Err(e)),
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: stripper <ignore-file>");
        std::process::exit(1);
    }
    let rules_path = &args[1];
    let ignore = Gitignore::new(rules_path).0;
    let mut dir = WalkDir::new(".");
    dir = dir.sort_by_key(|a| a.file_name().to_owned());
    for entry in process(dir.into_iter(), &ignore) {
        let dir_entry = entry.expect("extracting dir. entry");
        println!("{}", dir_entry.path().to_str().expect("string encoding"));
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::{Entry, process};
    use ignore::gitignore::Gitignore;
    use ignore::gitignore::GitignoreBuilder;

    #[derive(Debug, Clone, PartialEq)]
    struct TestEntry {
        path: PathBuf,
        is_dir: bool,
    }

    impl TestEntry {
        fn file(name: String) -> Self {
            TestEntry {
                path: name.into(),
                is_dir: false,
            }
        }
        fn dir(name: String) -> Self {
            TestEntry {
                path: name.into(),
                is_dir: true,
            }
        }
    }

    impl Entry for TestEntry {
        fn path(&self) -> &Path {
            &self.path
        }
        fn is_dir(&self) -> bool {
            self.is_dir
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    struct TestError;

    fn build_ignore(rules: Vec<String>) -> Gitignore {
        let mut builder = GitignoreBuilder::new(".");
        for rule in rules {
            builder.add_line(None, rule.as_str()).expect("adding rule");
        }
        builder.build().expect("building rule engine")
    }

    fn process_and_collect(
        files: Vec<TestEntry>,
        ignore: &Gitignore,
    ) -> Vec<Result<TestEntry, TestError>> {
        let oked = files.into_iter().map(|e| Ok(e));
        process(oked, ignore).collect()
    }

    #[test]
    fn test_process() {
        {
            let rules = build_ignore(vec![]);
            let files = vec![];
            let got = process_and_collect(files, &rules);
            let want = vec![];
            assert_eq!(got, want);
        }
        {
            let rules = build_ignore(vec![]);
            let files = vec![
                TestEntry::dir("./bar".to_string()),
                TestEntry::file("./foo".to_string()),
            ];
            let got = process_and_collect(files, &rules);
            let want = vec![];
            assert_eq!(got, want);
        }
        {
            let rules = build_ignore(vec!["bar".to_string(), "foo".to_string()]);
            let files = vec![
                TestEntry::dir("./bar".to_string()),
                TestEntry::file("./foo".to_string()),
            ];
            let got = process_and_collect(files, &rules);
            let want = vec![
                Ok(TestEntry::dir("./bar".to_string())),
                Ok(TestEntry::file("./foo".to_string())),
            ];
            assert_eq!(got, want);
        }
        {
            let rules = build_ignore(vec!["bar".to_string(), "foo".to_string()]);
            let files = vec![
                TestEntry::dir("./bar".to_string()),
                TestEntry::file("./bar/foo".to_string()),
            ];
            let got = process_and_collect(files, &rules);
            let want = vec![
                Ok(TestEntry::dir("./bar".to_string())),
                Ok(TestEntry::file("./bar/foo".to_string())),
            ];
            assert_eq!(got, want);
        }
    }
}
