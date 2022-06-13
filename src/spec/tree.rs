use std::fmt::{Display, Debug, Formatter};
use std::io::Read;                                                                                                                                                                                                                                                                                                      ` =
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug)]
struct Tree {
    node: Node,
    children: Option<Box<Vec<Tree>>>,
    parent: Option<Tree>,
}

enum Node {
    SpecDir(NodeDesc),
    NormalDir(NodeDesc),
    NormalFile(NodeDesc, Option<NodeData>),
    GlobalDir(NodeDesc),
    GlobalModule(NodeDesc, Option<NodeData::Yaml>),
    ModuleDir(NodeDesc),
    ModuleFile(NodeDesc, Option<NodeData::Yaml>),
    ProfileDir(NodeDesc),
    Profile(NodeDesc, Option<NodeData::Yaml>),
}

struct NodeDesc {
    path: Path,
    name: String,
    level: usize,
}

enum NodeData {
    Txt,
    Json,
    Yaml,
    Csv,
}

#[allow(dead_code)]
impl Tree {
    fn new(spec_dir: &Path) -> Result<Tree, String> {
        if !spec_dir.exists() || !spec_dir.is_dir() {
            return Err(String::from("invalid spec dir"));
        };


        fn parse(node: &Tree, path: &Path, level: usize) -> Result<(), String> {
            if !path.is_dir() {
                return Err(String::from("parse need a direct instead of file"));
            }

            let entries = match path.read_dir() {
                Ok(entries) => {
                    entries
                }
                Err(e) => return Err(e.to_string()),
            };

            for entry in entries {

            }
        }
    }
}

