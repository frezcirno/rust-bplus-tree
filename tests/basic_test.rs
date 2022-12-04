use rust_bplus_tree::bp_tree::BPTree;

#[test]
fn hello_test() {
    let mut bptree = BPTree::<3, u32, u32>::new();
    bptree.insert(1, 1);
    print!("insert 1 and 2\n");
    bptree.insert(2, 2);
    bptree.print();

    print!("insert 3\n");
    bptree.insert(3, 3);
    bptree.print();

    print!("insert 4\n");
    bptree.insert(4, 4);
    bptree.print();
}
