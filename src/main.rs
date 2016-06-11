extern crate rand;
use rand::Rng;

fn tiny_sort<T: Copy + PartialOrd>(x: &mut [T]) {
	if x.len() <= 1 { return; }
	if x.len() == 2 {
		if x[0] > x[1] { x.swap(0, 1); }
		return;
	}
	if x.len() == 3 {
		for &(i,j) in [(0,1),(1,2),(0,1)].iter() {
			if x[i] > x[j] { x.swap(i, j); }
		}
		return;
	}
	
	for &(i,j) in [(0,1),(1,2),(2,3),(1,2),(0,1),(1,2)].iter() {
		if x[i] > x[j] { x.swap(i, j); }
	}
}

fn merge_sort<T: Copy + PartialOrd>(x: &mut [T]) {
	if x.len() <= 4 {
		tiny_sort(x);
		return;
	}
	
	let m = x.len() / 2;
	{
		let (mut left, mut right) = x.split_at_mut(m);
		merge_sort(&mut left);
		merge_sort(&mut right);
	}
	
	let mut i = 0;
	let mut j = m;
	
	let mut y: Vec<T> = Vec::with_capacity(x.len());
	
	while i < m && j < x.len() {
		if x[i] < x[j] {
			y.push(x[i]);
			i += 1;
		} else {
			y.push(x[j]);
			j += 1;
		}
	}
	while i < m {
		y.push(x[i]);
		i += 1;
	}
	while j < x.len() {
		y.push(x[j]);
		j += 1;
	}
	
	x.copy_from_slice(&y);
}

fn quick_sort<T: Copy + PartialOrd, R: rand::Rng>(x: &mut [T], rng: &mut R) {

	if x.len() <= 8 {
		merge_sort(x);
		return;
	}
	
	let n = x.len();
	let p = rng.gen_range(0, x.len());
	x.swap(n - 1, p);
	let p = x[x.len() - 1];
	
	let mut j = 0;
	
	for i in 0..x.len()-1 {
		if x[i] <= p {
			x.swap(i, j);
			j += 1;
		}
	}
	x.swap(j, n - 1);
	
	let (mut lteq, mut gt) = x.split_at_mut(j+1);
	let (mut lt, _) = lteq.split_at_mut(j);
	quick_sort(&mut lt, rng);
	quick_sort(&mut gt, rng);
}

#[derive(Clone,Copy,Debug)]
struct A(i32);

static mut N: i32 = 0;

impl std::cmp::PartialEq for A {
	fn eq(&self, other: &A) -> bool {
		self.0 == other.0
	}
}

impl std::cmp::PartialOrd for A {
	fn partial_cmp(&self, other: &A) -> Option<std::cmp::Ordering> {
		unsafe {
			N += 1;
		}
		self.0.partial_cmp(&other.0)
	}
}

fn main() {
	//let mut rng = rand::weak_rng();
	//let mut rng = rand::XorShiftRng::new_unseeded();	
	let seed: &[_] = &[1, 2, 3, 50];
	let mut rng: rand::StdRng = rand::SeedableRng::from_seed(seed);

	let mut x = Vec::new();
	
	for _ in 0..3 {
		x.push(A(rng.gen_range(-10, 10)));
	}
	
	quick_sort(&mut x, &mut rng);
	
	unsafe {
		println!("{} compare done", N);
	}

	for i in 0..x.len()-1 {
		if x[i] > x[i+1] {
			println!("sort error");
		}
	}
	
}
