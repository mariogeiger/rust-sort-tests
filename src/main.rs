extern crate time;
extern crate rand;
extern crate gnuplot;

mod heap;
mod merge;
mod quick;

use gnuplot::{Color, Caption, AxesCommon};
use rand::Rng;


struct A(i32);

static mut NCMP: usize = 0;
static mut NCPY: usize = 0;

impl std::cmp::PartialEq for A {
	fn eq(&self, other: &A) -> bool {
		self.0 == other.0
	}
}
impl std::cmp::Eq for A {}

impl std::cmp::PartialOrd for A {
	fn partial_cmp(&self, other: &A) -> Option<std::cmp::Ordering> {
		unsafe {
			NCMP += 1;
		}
		self.0.partial_cmp(&other.0)
	}
}
impl std::cmp::Ord for A {
	fn cmp(&self, other: &A) -> std::cmp::Ordering {
		unsafe {
			NCMP += 1;
		}
		self.0.cmp(&other.0)
	}
}

impl std::fmt::Debug for A {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Clone for A {
	fn clone(&self) -> A {
		unsafe { NCPY += 1; }
		A(self.0)
	}
}

impl Copy for A {
}

// repeat r times the bench with a vector of length n
fn bench<F>(n: usize, r: usize, f: F) -> (f64, usize, usize) 
	where F: Fn(&mut [A]) {
	let mut rng = rand::weak_rng();
	let mut x = Vec::new();
	for _ in 0..r {
		x.push(Vec::new());
		for _ in 0..n {
			x.last_mut().unwrap().push(A(rng.gen_range(i32::min_value(), i32::max_value())));
		}
	}
	
	unsafe {
		NCMP = 0;
		NCPY = 0;
	}
	
	let t0 = time::precise_time_s();
	for i in 0..r {
		f(&mut x[i]);
	}
	let t1 = time::precise_time_s();

	let (ncmp, ncpy);
	unsafe {
		ncmp = NCMP;
		ncpy = NCPY;
	}
	
	if n > 1 {
		for i in 0..r {
			for j in 0..n-1 {
				if x[i][j] > x[i][j+1] {
					println!("sort error");
				}
			}
		}
	}
	
	((t1 - t0) / r as f64, ncmp / r, ncpy / r)
}

fn chain_bench<F>(nmax: usize, nstep: usize, rep: usize, f: F) -> (Vec<usize>, Vec<f64>, Vec<usize>, Vec<usize>) 
	where F: Fn(&mut [A]) {
	
	let mut ns = Vec::new();
	let mut ts = Vec::new();
	let mut cmps = Vec::new();
	let mut cpys = Vec::new();
	
	let mut n = 0;
	while n < nmax {
		let (t, cmp, cpy) = bench(n, rep, &f);
		
		ns.push(n);
		ts.push(t);
		cmps.push(cmp);
		cpys.push(cpy);
		
		n += nstep;
	}
	
	(ns, ts, cmps, cpys)
}

fn main() {
	let (nmax, nstep, rep) = (10000, 63, 21);

	let mut fg_time = gnuplot::Figure::new();
	let mut fg_cmp = gnuplot::Figure::new();
	let mut fg_cpy = gnuplot::Figure::new();
	{
		let mut a_time = fg_time.axes2d()
		.set_title("time", &[]);
		
		let mut a_cmp = fg_cmp.axes2d()
		.set_title("compare", &[]);
		
		let mut a_cpy = fg_cpy.axes2d()
		.set_title("copies", &[]);


		let (ns, ts, cmps, cpys) = chain_bench(nmax, nstep, rep, 
			quick::quick_sort);
			
		a_time.points(&ns, &ts, &[Color("red"), Caption("quick")]);
		a_cmp.points(&ns, &cmps, &[Color("red"), Caption("quick")]);
		a_cpy.points(&ns, &cpys, &[Color("red"), Caption("quick")]);

		let (ns, ts, cmps, cpys) = chain_bench(nmax, nstep, rep, 
			heap::heap_sort);
			
		a_time.points(&ns, &ts, &[Color("blue"), Caption("heap")]);
		a_cmp.points(&ns, &cmps, &[Color("blue"), Caption("heap")]);
		a_cpy.points(&ns, &cpys, &[Color("blue"), Caption("heap")]);

		let (ns, ts, cmps, cpys) = chain_bench(nmax, nstep, rep, 
			merge::merge_sort);
			
		a_time.points(&ns, &ts, &[Color("green"), Caption("merge")]);
		a_cmp.points(&ns, &cmps, &[Color("green"), Caption("merge")]);
		a_cpy.points(&ns, &cpys, &[Color("green"), Caption("merge")]);

		let (ns, ts, cmps, cpys) = chain_bench(nmax, nstep, rep, 
			|x| x.sort());
			
		a_time.points(&ns, &ts, &[Color("black"), Caption("std")]);
		a_cmp.points(&ns, &cmps, &[Color("black"), Caption("std")]);
		a_cpy.points(&ns, &cpys, &[Color("black"), Caption("std")]);
	}
	fg_time.show();
	fg_cmp.show();
	fg_cpy.show();
}
