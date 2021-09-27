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

	pub fn insert(&mut self, val: T) -> usize {
		if self.size() == 0 {
			self.node(val)
		} else {
			let mut cur = &self.arena[0];
			let (parent_id, dir) = loop {
				cur = match val.cmp(&cur.val) {
					Ordering::Less => match cur.left {
						None => break (cur.idx, true),
						Some(i) => &self.arena[i],
					},
					Ordering::Equal => return cur.idx,
					Ordering::Greater => match cur.right {
						None => break (cur.idx, false),
						Some(i) => &self.arena[i],
					},
				}
			};
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

#[test]
fn bst_insert_root() {
	let mut t = ArenaTree::default();
	let root_id = t.insert(0usize);
	assert_eq!(t.size(), 1);
	assert_eq!(root_id, 0);

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
