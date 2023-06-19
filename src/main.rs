use std::time::Instant;
use tinyrand::{StdRand, Rand};

fn insert(n: usize, tree: &sled::Tree, rng: &mut StdRand) -> Vec<u16> {
    (0..n).map(|i| { 
        let j = rng.next_u16();
        tree.insert(j.to_le_bytes(), &i.to_le_bytes()).expect("moo");
        return j;
    }).collect()
}

fn read(keys: &Vec<[u8; 2]>, tree: &sled::Tree) {
    keys.iter().for_each(|key| { tree.get(key).expect("oh no!"); } )
}

fn test(db: &sled::Db, n: usize, tree: &str, rng: &mut StdRand) -> u128 {
    let tree = db.open_tree(tree).expect("mooo");
    let written: Vec<u16> = insert(n, &tree, rng);
    let vec1 = (n/10..n/5).zip(written.iter()).map(|i| i.1.to_le_bytes()).collect();
    let vec2 = (n/10..n/5).zip(written.iter()).map(|i| i.1.to_le_bytes()).collect();
    let vec3 = (n/10..n/5).zip(written.iter()).map(|i| i.1.to_le_bytes()).collect();
    let now = Instant::now();
    read(&vec1, &tree);
    read(&vec2, &tree);
    read(&vec3, &tree);
    now.elapsed().as_nanos() / (vec1.len() + vec2.len() + vec3.len()) as u128
}

fn main() {
    let db = sled::open("/mnt/dbdbdb").expect("mooo");
    let mut rng = StdRand::default();
    println!("100 entries, average read time: {}ns", test(&db, 100, "hundred", &mut rng));
    println!("1k entries, average read time: {}ns", test(&db, 1000, "thousand", &mut rng));
    println!("10k entries, average read time: {}ns", test(&db, 10000, "tenthousand", &mut rng));
    println!("1M entries, average read time: {}ns", test(&db, 1000000, "million", &mut rng));
}
