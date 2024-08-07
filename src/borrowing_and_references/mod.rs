
pub fn owner_function(){
    let mut mystring_1 =String::from("Hello!!, I am Rihana");
    borrow_function(&mystring_1);
    mutable_borrow_function(&mut mystring_1);
    println!("{}", mystring_1);
}

pub fn borrow_function(mystring_2: &String){
    println!("1,{}", mystring_2);
}

pub fn mutable_borrow_function(mystring_3: &mut String) {
    mystring_3.push_str(", after hanky-panky");
}