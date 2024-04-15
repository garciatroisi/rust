fn main() {
    references();
    mutable_references();
}

fn references() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // *x reads the heap value, so a = 1
    *x += 1; // *x on the left-side modifies the heap value,
             //     so x points to the value 2

    println!("x element is {}", x);
    println!("a element is {}", a);

    let r1: &Box<i32> = &x; // r1 points to x on the stack
    println!("r1 element is {}", r1);
    let b: i32 = **r1; // two dereferences get us to the heap value

    println!("b element is {}", b);

    let r2: &i32 = &*x; // r2 points to the heap value directly
    let c: i32 = *r2; // so only one dereference is needed to read it

    println!("c element is {}", c); 
}

fn mutable_references() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
}
