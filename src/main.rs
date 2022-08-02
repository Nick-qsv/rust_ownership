fn main() {
    let v = vec![1,2,3];
    //V owns the memory stored in the vector
    //vec data is on the heap though

    // let v2 = v;
    //only a single variable can own the memory at any time.
    //copying a pointer, so you have to two pointers to the same content
    //this can introduce a race condition.  so only one pointer allowed.

    let u = 1;
    let u2 = u;

    println!("u = {}", u);
    //works for primitives, because we can copy, it's stored on the stack.  no pointers to data
    
    let print_vector = |x:Vec<i32>| -> Vec<i32>{
        println!("{:?}", x);
        x
    };
    //borrow and then return the vector before the end.  you can printline it and then return it

    let vv = print_vector(v);
    println!("{}", vv[0]);

}
