
struct SparseVector {
    idxs: Vec<usize>,
    data: Vec<i32>
}

/**
  * `&self` means the method takes an immutable reference
  * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        let mut idxs = vec!();
        let mut data  = vec!();

        for (idx, n) in nums.into_iter().enumerate() {
            if n == 0 {
                continue;
            }

            idxs.push(idx);
            data.push(n);
        }

        SparseVector {
            idxs,
            data
        }
    }

    // Return the dotProduct of two sparse vectors
    fn dot_product(&self, vec: SparseVector) -> i32 {
        let mut result = 0;

        for (pos, idx) in self.idxs.iter().enumerate() {
            if !vec.idxs.contains(idx) {
                continue
            }


            let v1 = self.data[pos];
            let pos2 = vec.idxs.iter().position(|i| i == idx).unwrap();
            let v2 = vec.data[pos2];
            result += v1 * v2;
        }

        result
    }
}

/**
 * Your SparseVector object will be instantiated and called as such:
 * let v1 = SparseVector::new(nums1);
 * let v2 = SparseVector::new(nums2);
 * let ans = v1.dot_product(v2);
 */
fn main() {
    let nums1 = vec![1,0,0,2,3];
    let nums2 = vec![0,3,0,4,0];

    let v1 = SparseVector::new(nums1);
    let v2 = SparseVector::new(nums2);
    let ans = v1.dot_product(v2);
    println!("{}", ans);
}