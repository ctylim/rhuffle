use rand::{thread_rng, Rng};

pub fn fisher_yates_shuffle_n(n: usize) -> Vec<usize> {
    info!("shuffling array with length {}", n);
    let mut v: Vec<usize> = Vec::with_capacity(n);
    for i in 0..n {
        v.push(i);
    }
    let mut rng = thread_rng();
    for i in (0..n).rev() {
        if i > 0 {
            let r: usize = rng.gen();
            v.swap(i, r % (i + 1));
        }
    }
    v
}
