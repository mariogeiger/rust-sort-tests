// add the last element of the slice onto the heap
pub fn heap_add<T: PartialOrd>(x: &mut [T]) {
    let mut i = x.len() - 1;

    loop {
        if i == 0 {
            return;
        }
        let j = (i - 1) / 2;
        if x[i] <= x[j] {
            return;
        }

        x.swap(i, j);
        i = j;
    }
}

// remove the head and put it at the end
pub fn heap_pop<T: PartialOrd>(x: &mut [T]) {
    let n = x.len() - 1; // size of the new heap
    x.swap(0, n); // pop out the root

    let mut i = 0;

    loop {
        let j = 2 * i + 1; // first child
        let k = j + 1; // second child

        if j >= n {
            // both childrens are out of bound
            return;
        }
        if x[j] > x[i] && (k == n || x[j] > x[k]) {
            x.swap(i, j);
            i = j; // take place of your first child
        } else if k < n && x[k] > x[i] {
            x.swap(i, k);
            i = k; // take place of your second child
        } else {
            return; // or stay where you are
        }
    }
}

pub fn heap_sort<T: PartialOrd>(x: &mut [T]) {
    for i in 2..x.len() + 1 {
        heap_add(&mut x[0..i]);
    }
    for i in (1..x.len() + 1).rev() {
        heap_pop(&mut x[0..i]);
    }
}
