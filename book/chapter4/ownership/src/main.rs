
// Swap the value of two integers by using the reference to the value and
// dereferencing it with the other value.
fn swap_by_ref(a: &mut i32, b: &mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn swap_by_val(mut a: i32, mut b: i32) -> (i32, i32) {
    (a, b) = (b, a);
    (a, b)
}


fn main() {
    println!("Hello, world!");
    let mut a: i32 = 10;
    let mut b: i32 = 8;
    println!("a = {}, b = {}", a, b);
    swap_by_ref(&mut a, &mut b);
    println!("a = {}, b = {}", a, b);
    (a, b) = swap_by_val(a, b);
    println!("a = {}, b = {}", a, b);
}
