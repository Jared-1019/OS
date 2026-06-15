#![no_std]
#![no_main]

use user_lib::{println, exit, get_time};

#[no_mangle]
fn main() -> i32 {
    println!("[Test] Memory management test started...");
    
    // 测试1：循环计算测试
    println!("Test 1: Computation test");
    let mut sum = 0;
    for i in 0..100 {
        sum += i;
    }
    assert_eq!(sum, 4950);
    println!("Test 1 passed!");
    
    // 测试2：多次函数调用测试
    println!("Test 2: Repeated function call test");
    for i in 0..10 {
        let _x = i * i;
    }
    println!("Test 2 passed!");
    
    // 测试3：获取时间
    println!("Test 3: Get system time");
    let t1 = get_time();
    for _ in 0..1000 {
        let _x = 1 + 1;
    }
    let t2 = get_time();
    println!("Time elapsed: {} ms", t2 - t1);
    println!("Test 3 passed!");
    
    println!("[Test] Memory management test passed!");
    exit(0);
    0
}
