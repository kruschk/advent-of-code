use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    rc::Rc,
};

#[derive(Debug, Eq, PartialEq)]
struct HeavenlyBody {
    name: String,
    depth: Option<usize>,
    children: Vec<Rc<RefCell<HeavenlyBody>>>,
}

impl HeavenlyBody {
    fn new(name: String) -> HeavenlyBody {
        Self {
            name,
            depth: None,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, hb: Rc<RefCell<HeavenlyBody>>) {
        self.children.push(hb);
    }
}

fn set_depths(root: &Rc<RefCell<HeavenlyBody>>, depth: usize) {
    let mut root = root.borrow_mut();
    root.depth = Some(depth);
    for hb in root.children.iter() {
        set_depths(hb, depth + 1);
    }
}

fn orbit_checksum(root: &Rc<RefCell<HeavenlyBody>>) -> usize {
    let root = root.borrow();
    let mut acc = 0;
    for hb in root.children.iter() {
        acc += orbit_checksum(hb);
    }
    acc + root.depth.unwrap()
}

fn path_to_body(root: &Rc<RefCell<HeavenlyBody>>, name: &str, path: &mut Vec<String>) -> bool {
    let root = root.borrow();
    if root.name == name {
        path.push(name.to_string());
        return true;
    }
    for hb in root.children.iter() {
        if path_to_body(hb, name, path) {
            path.push(root.name.to_string());
            return true;
        }
    }
    false
}

fn get_orbits_from_file(path: &Path) -> HashMap<String, Rc<RefCell<HeavenlyBody>>> {
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);
    let mut hm = HashMap::new();
    for line in f.lines() {
        let line = line.unwrap();
        let mut iterator = line.split(')');
        let parent_name = iterator.next().unwrap();
        let child_name = iterator.next().unwrap();
        if hm.get(child_name).is_none() {
            let child = Rc::new(RefCell::new(HeavenlyBody::new(child_name.to_string())));
            hm.insert(child_name.to_string(), child);
        }
        let child = hm.get(child_name).unwrap();
        if let Some(parent) = hm.get(parent_name) {
            parent.borrow_mut().add_child(child.clone());
        } else {
            let parent = Rc::new(RefCell::new(HeavenlyBody::new(parent_name.to_string())));
            parent.borrow_mut().add_child(child.clone());
            hm.insert(parent_name.to_string(), parent);
        }
    }
    let root = hm.get("COM").unwrap();
    set_depths(&root, 0);
    hm
}

fn get_distance(root: &Rc<RefCell<HeavenlyBody>>) -> usize {
    let mut path_you = Vec::new();
    path_to_body(root, "YOU", &mut path_you);
    let mut path_san = Vec::new();
    path_to_body(root, "SAN", &mut path_san);
    let mut hs = HashSet::new();
    for elem in path_you.iter() {
        hs.insert(elem.clone());
    }
    for elem in path_san.iter() {
        hs.insert(elem.clone());
    }
    for (elem_you, elem_san) in path_you.into_iter().rev().zip(path_san.into_iter().rev()) {
        if elem_you == elem_san {
            hs.remove(&elem_you);
        }
    }
    hs.len() - 2
}

fn main() {
    let hm = get_orbits_from_file(Path::new("input.txt"));
    let root = hm.get("COM").unwrap();
    println!("Orbit checksum: {}", orbit_checksum(root));
    let dist = get_distance(root);
    println!("Distance to Santa: {}", dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let hm = get_orbits_from_file(Path::new("test0.txt"));
        let root = hm.get("COM").unwrap();
        assert_eq!(42, orbit_checksum(root));
    }

    #[test]
    fn test1() {
        let hm = get_orbits_from_file(Path::new("test1.txt"));
        let root = hm.get("COM").unwrap();
        assert_eq!(4, get_distance(root));
    }
}
