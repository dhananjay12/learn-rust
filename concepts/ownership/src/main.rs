fn main() {


    let s1 = String::from("abc");
   

    // let s2 = s1;
    // print!("{}", s1); // Error - borrow of moved value s1

    let s2 = s1.clone();
    println!("{}", s1); // Works fine

    let s3 = String::from("123");

    // do_stuff1(s3);
    // print!("{}", s3); // Error - moved

    do_stuff2(&s3);
    println!("{}", s3); // Works fine

    let mut s4 = String::from("xyz");

    // do_stuff3(&s4);
    // println!("{}", s4); //Error 

    do_stuff4(&mut s4);
    println!("{}", s4); // Works fine
}

// fn do_stuff1(s : String){
//     s;
// }

fn do_stuff2(s : &String){
    println!("{}", s);
}

// fn do_stuff3(s : &String){
//     s.insert_str(0, "Hi, "); // Error - Reference defaults to immutable, even if the value refereced is mutable
//     println!("{}", s);
// }

fn do_stuff4(s : &mut String){
    s.insert_str(0, "Hi, "); // Works fine
    println!("{}", s);
}