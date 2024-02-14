use crate::product_not_self::product_except_self;

pub mod two_sum;
pub mod buy_sell_stock;
pub mod array_dupes;
pub mod product_not_self;
pub mod max_subarray;
pub mod max_product;
pub mod min_rotated_array;
pub mod search_rotated_array;
pub mod three_sum;
pub mod max_container;
pub mod binary_sum;
pub mod hamming_weight;
pub mod counting_bits;
pub mod missing_number;
pub mod reverse_bits;
pub mod climbing_stairs;
pub mod coin_change;

fn main() {
    println!("Hello, world!");
    let test = vec![3,4,5,6];
    
    product_except_self(test);
}
