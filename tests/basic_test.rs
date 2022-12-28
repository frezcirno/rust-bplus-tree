use rust_bplus_tree::bp_tree::BPTree;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

#[test]
fn insert_naive_test() {
    let mut bptree = BPTree::<3, u32, u32>::new();

    println!("insert 2 and 5");
    bptree.insert(2, 2);
    bptree.insert(5, 5);
    println!("{:?}", bptree);

    println!("insert 6");
    bptree.insert(6, 6);
    println!("{:?}", bptree);

    println!("insert 3");
    bptree.insert(3, 3);
    println!("{:?}", bptree);

    println!("insert 7");
    bptree.insert(7, 7);
    println!("{:?}", bptree);

    println!("insert 1");
    bptree.insert(1, 1);
    println!("{:?}", bptree);

    println!("insert 4");
    bptree.insert(4, 4);
    println!("{:?}", bptree);
}

#[test]
fn muti_threads(){
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            ("new",0, 0),
            ("insert", 3,3),
            ("insert", 2,2),
            // ("delete", 3,3),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    let mut bptree = BPTree::<3, u32, u32>::new();

    for received in rx {
        
        match received.0 {
            "new" => {
                println!("A new tree!");
                bptree = BPTree::<3, u32, u32>::new();
                println!("{:?}", bptree);
            },
            "insert" => {
                println!("insert");
                bptree.insert(received.1, received.2);
                println!("{:?}", bptree);
            },
            "delete" => {
                println!("delete");
                bptree.remove(&received.1);
                println!("{:?}", bptree);
            },
            _ => panic!("this operation is wrong"),
        }
    }

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


#[test]
fn remove_test2() {
    let mut bptree = BPTree::<6, u32, u32>::new();

    let vv = vec![1, 2, 3, 5, 44, 197, 438, 50, 60 ];
    println!("insert [1, 2, 3, 5, 44, 197, 438]");
    for i in vv{
        bptree.insert(i, i);
    }
    println!("{:?}", bptree);

    println!("remove 2");
    bptree.remove(&2);
    println!("{:?}", bptree);
}

