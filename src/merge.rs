#[allow(dead_code)]
fn merge<T: Copy + PartialOrd>(x1: &[T], x2: &[T], y: &mut [T]) {
	assert_eq!(x1.len() + x2.len(), y.len());
	let mut i = 0;
	let mut j = 0;
	let mut k = 0;
	while i < x1.len() && j < x2.len() {
		if x1[i] < x2[j] {
			y[k] = x1[i];
			k += 1;
			i += 1;
		} else {
			y[k] = x2[j];
			k += 1;
			j += 1;
		}
	}
	if i < x1.len() {
		y[k..].copy_from_slice(&x1[i..]);
	}
	if j < x2.len() {
		y[k..].copy_from_slice(&x2[j..]);
	}
}

#[allow(dead_code)]
pub fn merge_sort<T: Copy + PartialOrd>(x: &mut [T]) {
	let n = x.len();
	let mut y = x.to_vec();
	let mut len = 1;
	while len < n {
		let mut i = 0;
		while i < n {
			if i + len >= n {
				y[i..].copy_from_slice(&x[i..]);
			} else if i + 2 * len > n {
				merge(&x[i..i+len], &x[i+len..], &mut y[i..]);				
			} else {
				merge(&x[i..i+len], &x[i+len..i+2*len], &mut y[i..i+2*len]);
			}
			i += 2 * len;
		}
		len *= 2;
		if len >= n {
			x.copy_from_slice(&y);
			return;
		}
		i = 0;
		while i < n {
			if i + len >= n {
				x[i..].copy_from_slice(&y[i..]);
			} else if i + 2 * len > n {
				merge(&y[i..i+len], &y[i+len..], &mut x[i..]);				
			} else {
				merge(&y[i..i+len], &y[i+len..i+2*len], &mut x[i..i+2*len]);
			}
			i += 2 * len;
		}
		len *= 2;
	}
}

#[allow(dead_code)]
pub fn merge_sort_rec<T: Copy + Ord>(x: &mut [T]) {
	let n = x.len();
	let m = n / 2;
	
	if n <= 1 {
		return;
	}
	
	merge_sort_rec(&mut x[0..m]);
	merge_sort_rec(&mut x[m..n]);
		
	let mut y: Vec<T> = x.to_vec();
	
	merge(&x[0..m], &x[m..n], &mut y[..]);
	
	x.copy_from_slice(&y);
}
