use rand::{thread_rng, Rng};

pub fn fisher_yates_shuffle_n(n: usize) -> Vec<usize> {
    info!("shuffling array with length {}", n);
    let mut v: Vec<usize> = Vec::with_capacity(n);
    for i in 0..n {
        v.push(i);
    }
    let mut rng = thread_rng();
    for i in (1..n).rev() {
        let r: usize = rng.gen();
        v.swap(i, r % (i + 1));
    }
    v
}

#[test]
fn fisher_yates_shuffle_chi_square_test() {
    let mut cnt = [[0u32; 10]; 10];
    for _ in 0..10000 {
        let v = fisher_yates_shuffle_n(10);
        for i in 0..10 {
            cnt[i][v[i]] += 1;
        }
    }
    let chis: Vec<f32> = cnt
        .iter()
        .map(|x| {
            let mut a = 0f32;
            for v in x.iter() {
                a += (*v as f32 - 1000f32) * (*v as f32 - 1000f32) / 1000f32;
            }
            a
        })
        .collect();
    for chi in chis {
        assert!(chi < 25.2);
    }
}
