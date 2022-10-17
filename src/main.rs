use std::path::Path;

use libmdbx::{Environment, NoWriteMap, WriteFlags};

fn main() {
    let env = Environment::<NoWriteMap>::new()
        .open(Path::new("_db"))
        .unwrap();

    // write
    {
        let txn = env.begin_rw_txn().unwrap();
        let db = txn.open_db(None).unwrap();
        for i in 0..100 {
            txn.put(
                &db,
                &format!("key{}", i),
                &format!("data{}", i),
                WriteFlags::empty(),
            )
            .unwrap();
            // delete
            if i > 50 {
                txn.del(&db, &format!("key{}", i), None).unwrap();
            }
        }
        txn.commit().unwrap();
    }
    // read
    {
        let txn = env.begin_ro_txn().unwrap();
        let db = txn.open_db(None).unwrap();
        for i in 0..100 {
            println!(
                "{:?}",
                txn.get::<Vec<u8>>(&db, format!("key{}", i).as_bytes())
                    .unwrap()
            );
        }
    }
}
