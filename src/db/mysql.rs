use rustorm::{EntityManagerMut, Pool};
use std::process::exit;

pub fn connect(db_url:&str) ->(Pool,EntityManagerMut) {
    let mut pool = Pool::new();
    pool.ensure(db_url).unwrap_or_else(|e| {
        println!("{:?}",e);
        exit(-401);
    });
    let mut em = pool.em_mut(db_url).unwrap_or_else(|e| {
        println!("{:?}",e);
        exit(-402);
    });
    (pool,em)
}