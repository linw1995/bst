use std::cmp::Ordering;

#[derive(Debug)]
pub struct Node<T> {
    idx: usize,
    val: T,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

#[derive(Debug, Default)]
pub struct ArenaTree<T> {
    root_id: usize,
    arena: Vec<Node<T>>,
}

impl<T> Node<T> {
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            left: None,
            right: None,
        }
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

#[derive(Debug)]
pub enum Traversal {
    // DFS
    NLR,
    LNR,
    LRN,
    NRL,
    RNL,
    RLN,
    // DFS end
    BFS,
}

impl<T> ArenaTree<T>
where
    T: Ord + Copy,
{
    fn node(&mut self, val: T) -> usize {
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        let mut t = Self {
            arena: vec![],
            root_id: 0,
        };
        for &val in v.iter() {
            t.insert(val);
        }
        t
    }

    pub fn size(&self) -> usize {
        self.arena.len()
    }

    pub fn search_parent(&mut self, val: T) -> Option<(usize, bool)> {
        if self.size() == 0 {
            None
        } else {
            let mut cur = &self.arena[self.root_id];
            loop {
                cur = match val.cmp(&cur.val) {
                    Ordering::Less => match cur.left {
                        None => break Some((cur.idx, true)),
                        Some(i) => &self.arena[i],
                    },
                    Ordering::Equal => {
                        break match cur.parent {
                            None => None,
                            Some(parent_id) => {
                                Some((parent_id, self.arena[parent_id].left == Some(cur.idx)))
                            }
                        }
                    }
                    Ordering::Greater => match cur.right {
                        None => break Some((cur.idx, false)),
                        Some(i) => &self.arena[i],
                    },
                }
            }
        }
    }

    pub fn search(&mut self, val: T) -> Option<usize> {
        match self.search_parent(val) {
            None => {
                if !self.arena.is_empty() && self.arena[self.root_id].val == val {
                    Some(self.root_id)
                } else {
                    None
                }
            }
            Some((parent_id, dir)) => {
                let parent = &self.arena[parent_id];
                if dir {
                    parent.left
                } else {
                    parent.right
                }
            }
        }
    }

    pub fn insert(&mut self, val: T) -> usize {
        match self.search_parent(val) {
            None => {
                if !self.arena.is_empty() && self.arena[self.root_id].val == val {
                    0
                } else {
                    self.node(val)
                }
            }
            Some((parent_id, dir)) => {
                {
                    let parent = &self.arena[parent_id];
                    if dir {
                        if parent.left.is_some() {
                            return parent.left.unwrap();
                        }
                    } else if parent.right.is_some() {
                        return parent.right.unwrap();
                    }
                }
                let id = self.node(val);
                {
                    let node = &mut self.arena[id];
                    node.parent.replace(parent_id);
                }
                {
                    let parent = &mut self.arena[parent_id];
                    if dir {
                        parent.left.replace(id);
                    } else {
                        parent.right.replace(id);
                    }
                }
                id
            }
        }
    }

    fn most_left(&self, id: usize) -> usize {
        let mut cur = &self.arena[id];
        loop {
            cur = match cur.left {
                Some(id) => &self.arena[id],
                None => break cur.idx,
            };
        }
    }

    /// delete may produce a gap in arena.
    pub fn delete(&mut self, val: T) -> bool {
        match self.search(val) {
            None => false,
            Some(id) => {
                let (parent_id, right_id, left_id) = {
                    let cur = &self.arena[id];
                    (cur.parent, cur.right, cur.left)
                };
                macro_rules! update_parent {
                    ($parent_id: expr, $id: expr, $original_id: expr) => {
                        match ($parent_id, $id) {
                            (None, None) => self.arena.clear(),
                            (None, Some(id)) => {
                                self.root_id = id;
                            }
                            (Some(parent_id), val) => {
                                let parent = &mut self.arena[parent_id];
                                if parent.left == Some($original_id) {
                                    parent.left = val;
                                } else {
                                    parent.right = val;
                                }
                            }
                        }
                    };
                }
                match (left_id, right_id) {
                    (None, None) => update_parent!(parent_id, None, id),
                    (Some(left_id), Some(right_id)) => {
                        let candidate_id = self.most_left(right_id);
                        update_parent!(parent_id, Some(candidate_id), id);
                        let (candidate_parent_id, candidate_right) = {
                            let candidate = &mut self.arena[candidate_id];
                            let candidate_right = candidate.right;
                            let candidate_parent_id = if right_id == candidate_id {
                                Some(candidate_id)
                            } else {
                                candidate.parent
                            };

                            candidate.right = Some(right_id);
                            candidate.left = Some(left_id);
                            candidate.parent = parent_id;

                            (candidate_parent_id, candidate_right)
                        };
                        update_parent!(candidate_parent_id, candidate_right, candidate_id);
                    }
                    (Some(left_id), None) => {
                        update_parent!(parent_id, Some(left_id), id);
                        self.arena[left_id].parent = parent_id;
                    }
                    (None, Some(right_id)) => {
                        update_parent!(parent_id, Some(right_id), id);
                        self.arena[right_id].parent = parent_id;
                    }
                }
                true
            }
        }
    }

    pub fn traversal(&self, typ: &Traversal) -> Vec<T> {
        self.traversal_map(typ, |x| x)
    }

    pub fn traversal_map(&self, typ: &Traversal, f: fn(T) -> T) -> Vec<T> {
        if self.arena.is_empty() {
            return vec![];
        }
        let mut path = Vec::with_capacity(self.size());
        match typ {
            Traversal::BFS => self.traversal_map_in_bfs(f, &mut path),
            _ => self.recursive_traversal_map_in_dfs(typ, f, Some(self.root_id), &mut path),
        }
        path
    }

    fn traversal_map_in_bfs(&self, f: fn(T) -> T, path: &mut Vec<T>) {
        use std::collections::VecDeque;
        let mut q = VecDeque::with_capacity(self.size());
        let mut cur = &self.arena[self.root_id];

        #[cfg(debug_assertions)]
        use std::collections::HashSet;
        #[cfg(debug_assertions)]
        let mut set = HashSet::with_capacity(self.size());

        loop {
            path.push(f(cur.val));

            #[cfg(debug_assertions)]
            if !set.insert(cur.idx) {
                break;
            }

            if cur.left.is_some() {
                q.push_back(cur.left.unwrap());
            }
            if cur.right.is_some() {
                q.push_back(cur.right.unwrap());
            }
            match q.pop_front() {
                Some(id) => cur = &self.arena[id],
                None => break,
            }
        }
    }

    fn recursive_traversal_map_in_dfs(
        &self,
        typ: &Traversal,
        f: fn(T) -> T,
        id: Option<usize>,
        path: &mut Vec<T>,
    ) {
        match id {
            None => {}
            Some(id) => {
                let node = &self.arena[id];
                macro_rules! R {
                    () => {
                        self.recursive_traversal_map_in_dfs(typ, f, node.right, path);
                    };
                }
                macro_rules! L {
                    () => {
                        self.recursive_traversal_map_in_dfs(typ, f, node.left, path);
                    };
                }
                macro_rules! N {
                    () => {
                        path.push(f(node.val));
                    };
                }
                macro_rules! invoke_marcos {
					($($name: ident),*) => {{
						$($name!();)*
					}};
				}
                match typ {
                    Traversal::NLR => invoke_marcos!(N, L, R),
                    Traversal::LNR => invoke_marcos!(L, N, R),
                    Traversal::LRN => invoke_marcos!(L, R, N),
                    Traversal::NRL => invoke_marcos!(N, R, L),
                    Traversal::RNL => invoke_marcos!(R, N, L),
                    Traversal::RLN => invoke_marcos!(R, L, N),
                    Traversal::BFS => unreachable!(),
                }
            }
        }
    }
}

#[test]
fn bst_insert_root() {
    let mut t = ArenaTree::default();
    let root_id = t.insert(0usize);
    assert_eq!(t.size(), 1);
    assert_eq!(root_id, 0);

    println!("arena: {:?}", t);
}

#[test]
fn bst_insert_same_root_twice() {
    let mut t = ArenaTree::default();
    let root_id = t.insert(0usize);
    assert_eq!(t.size(), 1);
    assert_eq!(root_id, 0);

    let new_id = t.insert(0usize);
    assert_eq!(t.size(), 1);
    assert_eq!(new_id, 0);

    println!("arena: {:?}", t);
}

#[test]
fn bst_insert_same_twice() {
    let mut t = ArenaTree::default();
    let root_id = t.insert(10usize);
    let left_id = t.insert(0usize);
    assert_eq!(t.size(), 2);
    assert_eq!(t.arena[left_id].parent.unwrap(), root_id);

    let new_id = t.insert(0usize);
    assert_eq!(new_id, left_id);
    assert_eq!(t.size(), 2);

    println!("arena: {:?}", t);
}

#[test]
fn bst_insert_less() {
    let mut t = ArenaTree::default();
    let root_id = t.insert(10usize);
    let left_id = t.insert(0usize);
    assert_eq!(t.size(), 2);
    assert_eq!(t.arena[left_id].parent.unwrap(), root_id);

    assert_eq!(t.arena[0].left.unwrap(), left_id);

    println!("arena: {:?}", t);
}

#[test]
fn bst_insert_greater() {
    let mut t = ArenaTree::default();
    let root_id = t.insert(0usize);
    let left_id = t.insert(10usize);
    assert_eq!(t.size(), 2);
    assert_eq!(t.arena[left_id].parent.unwrap(), root_id);

    assert_eq!(t.arena[0].right.unwrap(), left_id);

    println!("arena: {:?}", t);
}

#[test]
fn bst_traversal() {
    let t = ArenaTree::from_vec(vec![2, 1, 3]);

    println!("arena: {:?}", t);

    let testcases = vec![
        (&Traversal::NLR, vec![2, 1, 3]),
        (&Traversal::LNR, vec![1, 2, 3]),
        (&Traversal::LRN, vec![1, 3, 2]),
        (&Traversal::NRL, vec![2, 3, 1]),
        (&Traversal::RNL, vec![3, 2, 1]),
        (&Traversal::RLN, vec![3, 1, 2]),
        (&Traversal::BFS, vec![2, 1, 3]),
    ];

    for (mode, expect) in testcases.iter() {
        println!("mode: {:?}, expect: {:?}", mode, expect);
        assert_eq!(&t.traversal(mode), expect);
    }
}

#[test]
fn bst_traversal_complex() {
    let t = ArenaTree::from_vec(vec![5, 1, 2, 4, 3]);

    println!("arena: {:?}", t);

    let testcases = vec![
        (&Traversal::NLR, vec![5, 1, 2, 4, 3]),
        (&Traversal::LNR, vec![1, 2, 3, 4, 5]),
        (&Traversal::LRN, vec![3, 4, 2, 1, 5]),
        (&Traversal::NRL, vec![5, 1, 2, 4, 3]),
        (&Traversal::RNL, vec![5, 4, 3, 2, 1]),
        (&Traversal::RLN, vec![3, 4, 2, 1, 5]),
        (&Traversal::BFS, vec![5, 1, 2, 4, 3]),
    ];

    for (mode, expect) in testcases.iter() {
        println!("mode: {:?}, expect: {:?}", mode, expect);
        assert_eq!(&t.traversal(mode), expect);
    }
}

#[test]
fn bst_delete_leaf() {
    let mut t = ArenaTree::from_vec(vec![4, 2, 6, 1, 3, 5, 7]);
    println!("arena: {:?}", t);

    assert_eq!(t.traversal(&Traversal::BFS), vec![4, 2, 6, 1, 3, 5, 7]);

    assert_eq!(t.delete(1), true);
    assert_eq!(t.traversal(&Traversal::BFS), vec![4, 2, 6, 3, 5, 7]);
    assert_eq!(t.delete(1), false);
}

#[test]
fn bst_delete_node() {
    let mut t = ArenaTree::from_vec(vec![4, 2, 6, 1, 3, 5, 7]);
    println!("arena: {:?}", t);

    assert_eq!(t.traversal(&Traversal::BFS), vec![4, 2, 6, 1, 3, 5, 7]);

    assert_eq!(t.delete(4), true);
    assert_eq!(t.traversal(&Traversal::BFS), vec![5, 2, 6, 1, 3, 7]);
    assert_eq!(t.delete(4), false);
}

#[test]
fn bst_delete_node_2() {
    let mut t = ArenaTree::from_vec(vec![4, 2, 6, 1, 3, 5, 7]);
    let testcases = vec![
        (4, vec![5, 2, 6, 1, 3, 7]),
        (5, vec![6, 2, 7, 1, 3]),
        (6, vec![7, 2, 1, 3]),
        (7, vec![2, 1, 3]),
        (2, vec![3, 1]),
        (3, vec![1]),
        (1, vec![]),
    ];
    for (val, expect) in testcases.iter() {
        println!("delete {:?}", val);
        assert_eq!(t.delete(*val), true);
        assert_eq!(t.traversal(&Traversal::BFS), *expect);
    }
}

#[test]
fn bst_most_left() {
    let t = ArenaTree::from_vec(vec![4, 2, 6, 1, 3, 5, 7]);
    println!("arena: {:?}", t);

    assert_eq!(t.most_left(0), 3);
    assert_eq!(t.most_left(1), 3);
    assert_eq!(t.most_left(2), 5);
}
