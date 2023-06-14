use std::collections::HashMap;

#[derive(Debug)]
struct SnapshotArray {
    changes: Vec<HashMap<i32, i32>>,
    snap_id: i32,
}


impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray { changes: vec![HashMap::new(); length as usize], snap_id: 0 }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.changes[index as usize].insert(self.snap_id, val);
    }

    fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        return self.snap_id - 1;
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let index_histories = &self.changes[index as usize];
        if let Some(value) = index_histories.get(&snap_id) {
            *value
        } else {
            let mut keys: Vec<&i32> = index_histories.keys().filter(|&&k| k < snap_id).collect();
            if keys.is_empty() {
                return 0;
            }

            keys.sort();
            *index_histories.get(&(keys[keys.len() - 1])).unwrap()
        }
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */

fn main() {
    let mut obj = SnapshotArray::new(2);
    obj.snap();
    obj.get(1, 0);
    obj.get(0, 0);
    obj.set(1, 8);
    obj.get(1, 0);
    obj.set(0, 20);
    obj.get(0, 0);
    obj.set(0, 7);

    // println!("{}", obj.get(0, 3));


}