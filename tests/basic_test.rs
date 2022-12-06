use rust_bplus_tree::bp_tree::BPTree;

#[test]
fn insert_naive_test() {
    let mut bptree = BPTree::<3, u32, u32>::new();

    println!("insert 1 and 2");
    bptree.insert(1, 1);
    bptree.insert(2, 2);
    println!("{:?}", bptree);

    println!("insert 3");
    bptree.insert(3, 3);
    println!("{:?}", bptree);

    println!("insert 4");
    bptree.insert(4, 4);
    println!("{:?}", bptree);

    println!("insert 5");
    bptree.insert(5, 5);
    println!("{:?}", bptree);
}

#[test]
fn remove_naive_test() {
    let mut bptree = BPTree::<3, u32, u32>::new();

    println!("insert [1, 2, 3, 4, 5]");
    bptree.insert(1, 1);
    bptree.insert(2, 2);
    bptree.insert(3, 3);
    bptree.insert(4, 4);
    bptree.insert(5, 5);
    println!("{:?}", bptree);

    println!("remove 2");
    bptree.remove(&2);
    println!("{:?}", bptree);
}
