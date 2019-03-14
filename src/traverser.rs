use std::fs::ReadDir;
use std::fs;
use std::path::PathBuf;

pub struct Traverser {
    root: PathBuf,
    dir: Option<ReadDir>,
    child: Option<Box<Traverser>>,
}

impl Traverser {
    pub fn new(root: PathBuf)-> Traverser {
        Traverser {
            root: root.to_owned(),
            dir: if root.is_dir() {
                Some(fs::read_dir(&root).expect("Permission error"))
            } else {
                None
            },
            child: None,
        }
    }
}

impl Iterator for Traverser {
    type Item = PathBuf;

    fn next(&mut self) -> Option<PathBuf> {
        if let Some(child) = self.child.as_mut() {
            if let Some(item) = child.next() {
                return Some(item)
            }

            self.child = None;
        }

        if let Some(dir) = self.dir.as_mut() {
            if let Some(entry) = dir.next() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    if let Some(os_str) = path.as_path().file_name() {
                        if os_str.to_str() == Some("node_modules") {
                            return Some(fs::canonicalize(&self.root).unwrap())
                        }
                    }
                    self.child = Some(Box::new(Traverser::new(path)));
                    self.next()
                } else {
                    self.next()
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
