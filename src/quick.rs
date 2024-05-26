pub fn quick_sort<T: PartialOrd>(x: &mut [T]) {
    let n = x.len();
    if n < 2 {
        return;
    }

    let p = n / 2;

    if x[n - 1] < x[0] {
        x.swap(0, n - 1);
    }
    if n == 2 {
        return;
    }

    if x[p] < x[0] {
        x.swap(0, p);
    }
    if x[n - 1] < x[p] {
        x.swap(p, n - 1);
    }
    if n == 3 {
        return;
    }

    x.swap(p, n - 1);

    let mut j = 0;

    for i in 0..x.len() - 1 {
        if x[i] <= x[n - 1] {
            x.swap(i, j);
            j += 1;
        }
    }
    x.swap(j, n - 1);

    quick_sort(&mut x[0..j]);
    quick_sort(&mut x[j + 1..n]);
}
