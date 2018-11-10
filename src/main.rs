#[macro_use]
extern crate lazy_static;
extern crate num;

use std::env;
use std::slice;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use num::bigint::BigUint;


lazy_static! {
    static ref CALLS: Arc<RwLock<HashMap<(u64, u64), BigUint>>> = {
        Arc::new(RwLock::new(HashMap::new()))
    };
}


fn num_combinations(num_items: u64, take: u64) -> BigUint {
    // let vec_len = items.len();
    if take == 1 {
         BigUint::from(num_items)
        // num_items
    }
    else if take == num_items {
         BigUint::from(1 as usize)
        // 1 as u64
    }
     else if take == num_items - 1 {
        // num_items
        BigUint::from(num_items)
    }
    else if take == 2 {
        BigUint::from(
            (0..num_items).fold(0, |acc, i| acc + i) as u64
        )
    }
    else {
        let res_match: Option<BigUint>;
        {
            let reader = CALLS.read().unwrap();
            match reader.get(&(num_items, take)) {
                Some(res) => {res_match = Some(res.clone())},
                None => {res_match = None}
            }
        }
        match res_match {
            Some(res) => res,
            None => {
                let res = 
                    num_combinations(num_items - 1, take - 1) 
                    + num_combinations(num_items - 1, take);
                let mut writer = CALLS.write().unwrap();
                writer.insert((num_items, take), res.clone());
                res
            }
        }
    }
}


fn print<T: Iterator + std::fmt::Debug>(thing: T) -> T{
    println!("{:?}", thing);
    thing
}


fn take_from<T: Clone + std::fmt::Debug>(items: &[T], take: u64) -> Vec<Vec<T>> {
    let item_len = items.len();
    if take == 1 {
        items.iter().map(|item| vec![item.clone()]).collect()
    }
    else if take == item_len  as u64 {
        vec![items.iter().map(|i| i.clone()).collect()]
    }
    else {
        // vec![items]
        println!("items: {:?}", items);
        items
            .into_iter()
            .enumerate()
            .map(|(index, _)| {
                // combine first item with all unique combinations from all other items
                println!("index: {}", index);
                let index_item = &items[index..index + 1];
                let rest = &items[index + 1..item_len];
                println!("index_item: {:?}", index_item);
                println!("rest: {:?}", rest);
                take_from(rest, take - 1)  // returns a vec of unique slices
                    .into_iter()
                    // combine unique slices with first integer item, yielding vectors of integers
                    .map(|uq_slice| {
                        let val = index_item
                            .iter()
                            .chain(uq_slice.iter())
                            .cloned()
                            .collect::<Vec<T>>();
                        println!("val: {:?}", val);
                        val
                    })
                    // collect slices into a vector
                    .collect::<Vec<Vec<T>>>()
            })
            .flatten()
            .collect::<Vec<Vec<T>>>()
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // let items = vec![1..args[1].parse::<u32>().unwrap()];
    // println!("{:?}", items);
    // let cached_fn = memoize_num_combs(num_combinations);
    //println!(
        //"{:?}", 
        //num_combinations(
            //args[1].parse::<u64>().unwrap(), 
            //args[2].parse::<u64>().unwrap()
        //).to_str_radix(10),
    //);
    let slice_one: &[i32] = &[1, 2, 3, 4, 5];
    let slice_two: &[i32] = &[3, 4];
    let items: Vec<u64> = (0..args[1].parse::<u64>().unwrap()).collect();
    let take = args[2].parse::<u64>().unwrap();
    println!("final: {:?}", take_from(&items, take));
}
