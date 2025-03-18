#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use containers::vector::Vector;
    use test::Bencher;

    // ===== NEW =====
    #[bench]
    fn bench_new(b: &mut Bencher) {
        b.iter(|| {
            let v: Vector<i32> = Vector::new(0, 10000);
            test::black_box(v);
        });
    }

    #[bench]
    fn bench_new_std(b: &mut Bencher) {
        b.iter(|| {
            let v: Vec<i32> = vec![0; 10000];
            test::black_box(v);
        });
    }

    // ===== APPEND =====
    #[bench]
    fn bench_append(b: &mut Bencher) {
        let mut v: Vector<i32> = Vector::new(0, 0);
        b.iter(|| {
            v.push(1);
            test::black_box(&v);
        });
        drop(v);
    }

    #[bench]
    fn bench_append_std(b: &mut Bencher) {
        let mut v: Vec<i32> = Vec::new();
        b.iter(|| {
            v.push(1);
            test::black_box(&v);
        });
    }

    // ===== INSERT =====
    #[bench]
    fn bench_insert(b: &mut Bencher) {
        let mut v: Vector<i32> = Vector::new(0, 10);
        b.iter(|| {
            v.insert(5, 1);
            test::black_box(&v);
        });
    }

    #[bench]
    fn bench_insert_std(b: &mut Bencher) {
        let mut v: Vec<i32> = vec![0; 10];
        b.iter(|| {
            v.insert(5, 1);
            test::black_box(&v);
        });
    }

    // ===== POP =====
    #[bench]
    fn bench_pop(b: &mut Bencher) {
        let mut v: Vector<i32> = Vector::new(0, 10000);
        b.iter(|| {
            v.pop();
            test::black_box(&v);
        });
    }

    #[bench]
    fn bench_pop_std(b: &mut Bencher) {
        let mut v: Vec<i32> = vec![0; 10000];
        b.iter(|| {
            v.pop();
            test::black_box(&v);
        });
    }

    // ===== INDEX =====
    #[bench]
    fn bench_index(b: &mut Bencher) {
        let v: Vector<i32> = Vector::new(0, 1000);
        b.iter(|| {
            let elem = v[500];
            test::black_box(elem);
        });
    }

    #[bench]
    fn bench_index_std(b: &mut Bencher) {
        let v: Vec<i32> = vec![0; 1000];
        b.iter(|| {
            let elem = v[500];
            test::black_box(elem);
        });
    }

    // ===== INDEX MUT =====
    #[bench]
    fn bench_index_mut(b: &mut Bencher) {
        let mut v: Vector<i32> = Vector::new(0, 1000);
        b.iter(|| {
            v[500] = 1;
            test::black_box(&v);
        });
    }
    #[bench]
    fn bench_index_mut_std(b: &mut Bencher) {
        let mut v: Vec<i32> = vec![0; 1000];
        b.iter(|| {
            v[500] = 1;
            test::black_box(&v);
        });
    }
}

