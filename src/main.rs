use crate::product_not_self::product_except_self;

pub mod two_sum;
pub mod buy_sell_stock;
pub mod array_dupes;
pub mod product_not_self;
pub mod max_subarray;
pub mod max_product;

fn main() {
    println!("Hello, world!");
    let test = vec![3,4,5,6];
    
    product_except_self(test);
}
