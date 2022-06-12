use std::fmt::{Display, Debug, Formatter};
use std::io::Read;                                                                                                                                                                                                                                                                                                      ` =
use std::path::Path;
use std::rc::{Rc, Weak};
use std::cell::{Ref, RefCell};

use crate::spec::spec::Spec;

#[allow(dead_code)]
#[derive(Debug)]
struct SpecTree {
    spec: Spec,
    data: Option<NodeData>,
    path: Box<Path>,
    name: String,
    kind: usize,
    is_dir: bool,
    level: isize,
    children: Option<Box<Vec<SpecTree>>>,
    parent: Option<SpecTree>,
    is_affiliated: bool,
}

enum NodeData {
    Txt,
    Json,
    Yaml,
    Csv,
}

#[allow(dead_code)]
impl SpecTree {
    fn build_root_node(specDir: &Path) -> Result<SpecTree, String> {
        if specDir.exists() && specDir.is_dir() {};

        Err("cannot build root node".to_string())
    }
}

