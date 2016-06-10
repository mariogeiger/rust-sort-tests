extern crate rand;

fn merge_sort<T: Copy + PartialOrd>(x: &mut [T]) {
	if x.len() <= 1 {
		return;
	}
	if x.len() == 2 {
		if x[0] > x[1] {
			x.swap(0, 1);
		}
		return;
	}
	if x.len() == 3 {
		if x[0] > x[1] {
			x.swap(0, 1);
		}
		if x[1] > x[2] {
			x.swap(1, 2);
		}
		if x[0] > x[1] {
			x.swap(0, 1);
		}
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

	if x.len() <= 6 {
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

fn main() {
	let mut x = vec![9,8,7,1,4,3,2,7,0,4,2,7,6,5,1,3,9,8,4,3,2,1];
	
	quick_sort(&mut x, &mut rand::weak_rng());
	
	println!("{:?}", x);
}
