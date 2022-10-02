fn main() {


    let s1 = String::from("abc");
   
    let x1 = 5;
    let y1 = x1; // Its a Copy and not move for primitive types that live on stack. (Because of a trait)
    println!("x is {} and y is {}", x1, y1);

    // let s2 = s1;
    // print!("{}", s1); // Error - borrow of moved value s1. Because istead of a shallow copy like in Java, it moves the value to the new reference.

    let s2 = s1.clone();
    println!("{}", s1); // Works fine

    let s3 = String::from("123");

    // do_stuff1(s3);
    // print!("{}", s3); // Error - moved

    let x2 = 5;
    do_stuff1_primitive(x2);
    println!("x after do_stuff1_primitive {}", x2); // Allowed because stack variable are copied not moved like String which is on heap

    let len = do_stuff2(&s3);
    println!("legth for s3 {} is {}", s3, len); // Works fine

    let mut s4 = String::from("xyz");

    // do_stuff3(&s4);
    // println!("{}", s4); //Error 

    do_stuff4(&mut s4);
    println!("{}", s4); // Works fine
}

fn do_stuff1(s : String){
    s;
}

fn do_stuff1_primitive(some_integer : i32){
    println!("some_integer {}", some_integer);
}

fn do_stuff2(s : &String) -> usize{
    println!("{}", s);
    s.len()
}

// fn do_stuff3(s : &String){
//     s.insert_str(0, "Hi, "); // Error - Reference defaults to immutable, even if the value refereced is mutable
//     println!("{}", s);
// }

fn do_stuff4(s : &mut String){
    s.insert_str(0, "Hi, "); // Works fine
    println!("{}", s);
}