fn main() {
    let s1 = String::from("abc1");
    do_stuff1(&s1);
    println!("{}", s1);

    reference_example1();

    // Example 2
    // let mut s2 = String::from("abc2");
    // do_stuff2(&s2);
    // println!("{}", s2);

    // Example 3
    let mut s3 = String::from("abc3");
    do_stuff3(&mut s3);
    println!("{}", s3);

    // Example 4
    let mut s4 = String::from("abc4");
    do_stuff4(&mut s4);
    println!("{}", s4);
}

fn do_stuff1(s: &String){ // Reference , gets moved to the funtion
    println!("do_stuff {}", s); 
}

// // Example 2
// fn do_stuff2(s: &String){
//     s.insert_str(0, "Hi"); // Error `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//     println!("do_stuff2 {}", s); 
// }

// Example 3
fn do_stuff3(s: &mut String){
    s.insert_str(0, "Hi "); // Change the original string too
    println!("do_stuff3 {}", s); 
}

// Example 4
fn do_stuff4(s: &mut String){
    (*s).insert_str(0, "Hi "); // Change the original string too
    println!("do_stuff4 {}", s); 
}

fn reference_example1(){
    let mut x = 10;

    let dom = &mut x; //Mutable reference to x, we can change x through dom

    //dom += 1; // Error `+=` cannot be applied to type `&mut {integer}`

    *dom += 1;

    println!("x is {}", x)
}