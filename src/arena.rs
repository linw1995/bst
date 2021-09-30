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
	NLR,
	LNR,
	LRN,
	NRL,
	RNL,
	RLN,
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

	pub fn size(&self) -> usize {
		self.arena.len()
	}

	pub fn search_parent(&mut self, val: T) -> Option<(usize, bool)> {
		if self.size() == 0 {
			None
		} else {
			let mut cur = &self.arena[0];
			loop {
				cur = match val.cmp(&cur.val) {
					Ordering::Less => match cur.left {
						None => break Some((cur.idx, true)),
						Some(i) => &self.arena[i],
					},
					Ordering::Equal => {
						break match cur.parent {
							None => None,
							Some(parent_id) => Some((
								parent_id,
								self.arena[parent_id].left
									== Some(cur.idx),
							)),
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
				if self.arena.len() > 0 && self.arena[0].val == val {
					Some(0)
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
				if self.arena.len() > 0 && self.arena[0].val == val {
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
					} else {
						if parent.right.is_some() {
							return parent.right.unwrap();
						}
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

	/// delete may produce a gap in arena.
	pub fn delete(&mut self, val: T) {
		todo!()
	}

	pub fn traversal(&self, typ: &Traversal) -> Vec<T> {
		self.traversal_map(typ, |x| x)
	}

	pub fn traversal_map(&self, typ: &Traversal, f: fn(T) -> T) -> Vec<T> {
		let mut path = Vec::with_capacity(self.size());
		self.recursive_traversal_map(typ, f, Some(0), &mut path);
		path
	}

	/// In-order Traversal (LNR)
	fn recursive_traversal_map(
		&self,
		typ: &Traversal,
		f: fn(T) -> T,
		id: Option<usize>,
		path: &mut Vec<T>,
	) {
		match id {
			None => return,
			Some(id) => {
				let node = &self.arena[id];
				macro_rules! R {
					() => {
						self.recursive_traversal_map(
							typ, f, node.right, path,
						);
					};
				}
				macro_rules! L {
					() => {
						self.recursive_traversal_map(
							typ, f, node.left, path,
						);
					};
				}
				macro_rules! N {
					() => {
						path.push(f(node.val));
					};
				}
				match typ {
					Traversal::NLR => {
						N!();
						L!();
						R!();
					}
					Traversal::LNR => {
						L!();
						N!();
						R!();
					}
					Traversal::LRN => {
						L!();
						R!();
						N!();
					}
					Traversal::NRL => {
						N!();
						R!();
						L!();
					}
					Traversal::RNL => {
						R!();
						N!();
						L!();
					}
					Traversal::RLN => {
						R!();
						L!();
						N!();
					}
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
	let mut t = ArenaTree::default();
	t.insert(2);
	t.insert(1);
	t.insert(3);

	println!("arena: {:?}", t);

	let testcases = vec![
		(&Traversal::NLR, vec![2, 1, 3]),
		(&Traversal::LNR, vec![1, 2, 3]),
		(&Traversal::LRN, vec![1, 3, 2]),
		(&Traversal::NRL, vec![2, 3, 1]),
		(&Traversal::RNL, vec![3, 2, 1]),
		(&Traversal::RLN, vec![3, 1, 2]),
	];

	for (mode, expect) in testcases.iter() {
		println!("mode: {:?}, expect: {:?}", mode, expect);
		assert_eq!(&t.traversal(mode), expect);
	}
}

#[test]
fn bst_traversal_complex() {
	let mut t = ArenaTree::default();
	t.insert(5);
	t.insert(1);
	t.insert(2);
	t.insert(4);
	t.insert(3);

	println!("arena: {:?}", t);

	let testcases = vec![
		(&Traversal::NLR, vec![5, 1, 2, 4, 3]),
		(&Traversal::LNR, vec![1, 2, 3, 4, 5]),
		(&Traversal::LRN, vec![3, 4, 2, 1, 5]),
		(&Traversal::NRL, vec![5, 1, 2, 4, 3]),
		(&Traversal::RNL, vec![5, 4, 3, 2, 1]),
		(&Traversal::RLN, vec![3, 4, 2, 1, 5]),
	];

	for (mode, expect) in testcases.iter() {
		println!("mode: {:?}, expect: {:?}", mode, expect);
		assert_eq!(&t.traversal(mode), expect);
	}
}
