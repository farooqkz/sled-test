use std::time::Instant;

fn insert(n: usize, mul: usize, tree: &sled::Tree) {
    (0..n).for_each(|i| { tree.insert((i*mul).to_le_bytes(), &i.to_le_bytes()).expect("moo"); });
}

fn read(keys: Vec<i32>, tree: &sled::Tree) {
    keys.iter().for_each(|key| { tree.get(key.to_le_bytes()).expect("oh no!"); } )
}

fn main() {
    let db = sled::open("/tmp/dbdbdb").expect("mooo");
    let tree = db.open_tree("default").expect("mooo");
    insert(100, 3, &tree);
    let vec = (0..10).map(|i| i * 3).collect();
    let now = Instant::now();
    read(vec, &tree);
    println!("About 100 entries, read 10 of them in {}ms", now.elapsed().as_micros());
    insert(10000, 10, &tree);
    let vec = (0..10).map(|i| i * 10).collect();
    let now = Instant::now();
    read(vec, &tree);
    println!("About 10k + 100 entries, read 10 of them in {}ms", now.elapsed().as_micros());
    insert(1000000, 100, &tree);
    let vec = (0..10).map(|i| i * 100).collect();
    let now = Instant::now();
    read(vec, &tree);
    println!("About 1M + 10k + 100 entries, read 10 of them in {}ms", now.elapsed().as_micros());
}
