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

	pub fn traversal(&self) -> Vec<T> {
		self.traversal_map(|x| x)
	}

	pub fn traversal_map(&self, f: fn(T) -> T) -> Vec<T> {
		let mut path = Vec::with_capacity(self.size());
		self.recursive_traversal_map(f, 0, &mut path);
		path
	}

	/// In-order Traversal (LNR)
	fn recursive_traversal_map(&self, f: fn(T) -> T, id: usize, path: &mut Vec<T>) {
		let node = &self.arena[id];
		if node.left.is_some() {
			self.recursive_traversal_map(f, node.left.unwrap(), path);
		}
		path.push(f(node.val));
		if node.right.is_some() {
			self.recursive_traversal_map(f, node.right.unwrap(), path);
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
fn bst_insert_same_twice() {
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
	let v = t.traversal();
	assert_eq!(v, vec![1, 2, 3]);
}
