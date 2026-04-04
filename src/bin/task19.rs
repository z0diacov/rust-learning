/*
 * PROJECT: RustyVFS
 * DESCRIPTION: An in-memory hierarchical file system simulation.
 * * GOALS:
 * 1. CORE TYPES:
 * - Create an enum `VNode`: represents either a `File` or a `Directory`.
 * - A `File` should have `name`, `content` (bytes), and `size`.
 * - A `Directory` should have `name` and a collection of `VNode`s.
 * * 2. SMART POINTERS:
 * - Use `Rc<RefCell<VNode>>` or `Arc<Mutex<VNode>>` for nodes. 
 * - Why? Because a directory "owns" its children, but you might want 
 * a "current working directory" pointer elsewhere.
 * * 3. TRAIT `FileSystemOperations`:
 * - Define methods for:
 * - `mkdir(path: &str) -> Result<(), VfsError>`
 * - `write_file(path: &str, content: Vec<u8>) -> Result<(), VfsError>`
 * - `read_file(path: &str) -> Result<Vec<u8>, VfsError>`
 * - `ls(path: &str) -> Result<Vec<String>, VfsError>`
 * - `rm(path: &str) -> Result<(), VfsError>`
 * * 4. PATH PARSING:
 * - Implement a helper to split "/home/user/docs" into components.
 * - Handle absolute and relative paths.
 * * 5. ERROR HANDLING:
 * - Create a custom `VfsError` enum: `NotFound`, `AlreadyExists`, `IsDirectory`, `NotADirectory`.
 * * 6. ADVANCED (The 500-line mark):
 * - Implement a `Search` trait that can find files by name pattern (regex-like)
 * recursively starting from a node.
 * - Add metadata: `created_at`, `modified_at` using `std::time::SystemTime`.
 * * 7. INTERFACE:
 * - Build a simple REPL (Read-Eval-Print Loop) in `main` so you can type 
 * commands like "mkdir docs", "ls", "cd .." in the console.
 */

use std::rc::Rc;
use std::cell::RefCell;
use std::result;

struct File {
    name: String,
    content: Vec<u8>,
    size: i128,
}

struct Directory {
    name: String,
    children: Vec<Rc<RefCell<VNode>>>,

}
enum VNode {
    File(File),
    Directory(Directory),
}

impl VNode {
    fn name(&self) -> String {
        match self {
            VNode::File(f) => f.name.clone(),
            VNode::Directory(d) => d.name.clone(),
        }
    }
}

enum VfsError {
    NotFound,
    AlreadyExists,
    IsDirectory,
    NotADirectory,
} 

struct FileSystem {
    root: Rc<RefCell<VNode>>,
    cwd: Rc<RefCell<VNode>>,
}
trait FileSystemOperations {
    // fn mkdir(&self, path: &str) -> Result<(), VfsError>;
    // fn write_file(&self, path: &str, content: Vec<u8>) -> Result<(), VfsError>;
    // fn read_file(&self, path: &str) -> Result<Vec<u8>, VfsError>;
    fn ls(&self, path: &str) -> Result<Vec<String>, VfsError>;
    // fn rm(&self, path: &str) -> Result<(), VfsError>;
}

impl FileSystemOperations for FileSystem {
    // fn internal_method(&self) {

    // }

    // fn mkdir(&self, path: &str) -> Result<(), VfsError> {

    // }
    // fn write_file(&self, path: &str, content: Vec<u8>) -> Result<(), VfsError> {

    // }
    // fn read_file(&self, path: &str) -> Result<Vec<u8>, VfsError> {

    // }
    fn ls(&self, path: &str) -> Result<Vec<String>, VfsError> {
        let mut result: Vec<String> = Vec::new();

        match &*self.root.borrow() {
            VNode::Directory(dir) => {
                for i in &dir.children {
                    let i_borrow = i.borrow();
                    let file_type_repr = match &*i_borrow {
                        VNode::Directory(_) => {
                            "D"
                        }
                        _ => {
                            "F"
                        }
                    };
                    result.push(format!("{} {}", file_type_repr, i_borrow.name()));
                }

                Ok(result)
            }
            _ => {
                Err(VfsError::NotADirectory)
            }
        }
    }
    // fn rm(&self, path: &str) -> Result<(), VfsError> {

    // }
}



fn main() {
    use std::rc::Rc;
    use std::cell::RefCell;

    let root = Rc::new(RefCell::new(VNode::Directory(Directory {
        name: "root".to_string(),
        children: vec![],
    })));

    if let VNode::Directory(dir) = &mut *root.borrow_mut() {
        dir.children.push(Rc::new(RefCell::new(VNode::File(File {
            name: "file1.txt".to_string(),
            content: vec![],
            size: 0,
        }))));

        dir.children.push(Rc::new(RefCell::new(VNode::File(File {
            name: "file2.log".to_string(),
            content: vec![],
            size: 0,
        }))));

        dir.children.push(Rc::new(RefCell::new(VNode::Directory(Directory {
            name: "docs".to_string(),
            children: vec![],
        }))));

        dir.children.push(Rc::new(RefCell::new(VNode::Directory(Directory {
            name: "images".to_string(),
            children: vec![],
        }))));
    }

    let fs = FileSystem {
        root: Rc::clone(&root),
        cwd: Rc::clone(&root),
    };

    match fs.ls("/") {
        Ok(list) => {
            for item in list {
                println!("{}", item);
            }
        }
        Err(_) => {
            println!("Error");
        }
    }
}