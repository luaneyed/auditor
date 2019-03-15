use std::fs::ReadDir;
use std::fs;
use std::path::PathBuf;

pub struct Traverser {
    root: PathBuf,
    is_root_target: bool,
    dir: Option<ReadDir>,
    cur_child: Option<Box<Traverser>>,
}

impl Traverser {
    pub fn new(root: PathBuf)-> Traverser {
        Traverser {
            root: root.to_owned(),
            is_root_target: root.join("package.json").exists(),
            dir: if root.is_dir() {
                Some(fs::read_dir(&root).expect(&format!("Permission error whilie reading {} directory", root.to_str().unwrap())))
            } else {
                None
            },
            cur_child: None,
        }
    }
}

impl Iterator for Traverser {
    type Item = PathBuf;

    fn next(&mut self) -> Option<PathBuf> {
        if self.is_root_target {
            self.is_root_target = false;
            return Some(fs::canonicalize(&self.root).unwrap())
        }

        if let Some(child) = self.cur_child.as_mut() {
            if let Some(item) = child.next() {
                return Some(item)
            }

            self.cur_child = None;
        }

        if let Some(dir) = self.dir.as_mut() {
            if let Some(entry) = dir.next() {
                let path = entry.unwrap().path();
                if !is_node_modules_directory(&path) {
                    self.cur_child = Some(Box::new(Traverser::new(path)));
                }
                self.next()
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn is_node_modules_directory(path: &PathBuf)-> bool {
    path.is_dir() && path.as_path().file_name().unwrap().to_str() == Some("node_modules")
}
