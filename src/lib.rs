pub fn hash(key: u64, num_buckets: i32) -> i32 {
    let mut b: i64 = -1;
    let mut j: i64 = 0;
    let mut k = key;
    while j < num_buckets as i64 {
        b = j;
        k = k.wrapping_mul(2862933555777941757).wrapping_add(1);
        j = ((b + 1) as f64 * ((1i64 << 31) as f64 / ((k >> 33) + 1) as f64)) as i64;
    }
    return b as i32;
}

#[test]
fn it_works() {
    for key in 0..100000 {
        let mut last_val = hash(key, 1);
        for num_buckets in 2..100 {
            let val = hash(key, num_buckets);
            assert!(
                val == last_val || val == num_buckets-1,
                "Expected hash({}, {}) to be {} or {} but got {}",
                key,
                num_buckets,
                last_val,
                num_buckets-1,
                val,
            );
            last_val = val;
        }
    }
}
