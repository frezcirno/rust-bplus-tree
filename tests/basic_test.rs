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

#[test]
fn remove_naive_test1() {
    let mut bptree = BPTree::<3, u32, u32>::new();

    println!("insert [4, 3, 2, 1, 0]");
    bptree.insert(4, 4);
    bptree.insert(3, 3);
    bptree.insert(2, 2);
    bptree.insert(1, 1);
    bptree.insert(0, 0);
    println!("{:?}", bptree);

    println!("remove 3");
    bptree.remove(&3);
    println!("{:?}", bptree);
}

#[test]
fn remove_test1() {
    let mut bptree = BPTree::<3, u32, u32>::new();

    println!("insert [1, 2, 3, 5, 44, 197, 438]");
    bptree.insert(1, 1);
    bptree.insert(2, 2);
    bptree.insert(3, 3);
    bptree.insert(5, 5);
    bptree.insert(44, 44);
    bptree.insert(197, 197);
    bptree.insert(438, 438);
    println!("{:?}", bptree);

    println!("remove 2");
    bptree.remove(&2);
    println!("{:?}", bptree);
}
