#![allow(unused_variables)]
#![allow(unused_mut)]
pub struct Playground {
}


impl Playground {
    pub fn new() -> Self{
        Self {

        }
    }

    pub fn test_playground(self) {

        /*

        Three rules of owner ship and borrowing

        1. ) every value is owned by a variable
        2. ) when the variable owner goes out of scope the value will be deallocated from memory
        3. ) there can be only one owner at a given time

         */

        // by default my Name is immutable, just like final in java
        let my_name = String::from("Byorn");

        //the owner of the value is moved to myAliasName
        let my_alisa_name = my_name;

        //uncommenting the below line wont compile, as you are trying to borrow the value my_name
        //only one owner can exist.
        //println!("{}", my_name);

        println!("{}", my_alisa_name);
        println! ("Testing ownership compilation");
    }

    //test passing the ownership to another function
    pub fn test_playground2() {

        let my_name = String::from("My String");
        Self::test_playground2_get_ownership_from_fn(my_name);
        // below line will fail compilation, as my_name does not exist.
        // println!("print my_name after the variable has been deallocated {}",my_name);
    }

    // the variable (pointer) takes ownership and gets dealocated once the function ends in the stack
    fn test_playground2_get_ownership_from_fn(str: String){
        println!("{}", str);
    }


    // how do we pass variables to functions without passing the ownership.
    // we pass the reference
    // this is called borrowing
    pub fn test_playground3() {

        let my_name = String::from("My String");
        Self::test_playground3_get_referendce_from_fn(&my_name);

        println!("print my_name after the variable reference has been passed  {}",my_name);
    }

    fn test_playground3_get_referendce_from_fn(str: &String){
        println!("{}", str);
    }


    /**
    In Rust you can have any immutable references
    But you can have only one mutable reference. (that means no other immutable references)

    Rust prevents data races at compile time
    **/
    pub fn test_any_immutable_ref_or_single_mutable_reference(){

        let mut my_mut_ref = String::from("my_mutable_ref");

        // can have many immutable references like below
        let immu_ref1 = &my_mut_ref;
        let immu_ref2 = &my_mut_ref;

        println! ("this compiles {} {}", immu_ref1, immu_ref2);

        // the ide and compilation will fail if you try to mutably borrow again
        // note mutably borring happens on the right hand side.
        // i.e. ref1 = mut &origial ref; [mut on the right hand side]

        // below line will fail compilateion
        //let mut mut_ref1 = mut &my_mut_ref;


        let mut my_orig_name = String::from("Bjorn");

        // the reference is borrowed mutably
        Self::some_fun( &mut my_orig_name);

        // this will become updated as well.
        println!("{}", my_orig_name);


        let  my_real_orig_name = String::from("Byorn");

        // the reference is borrowed immutably
        Self::some_fun_imu_ref( &my_real_orig_name);

        println!("original value not changed {}", my_real_orig_name);
;    }

    //the value in the reference is pointing to is updated.
    fn some_fun(my_name_to_mutate:  &mut String){

        my_name_to_mutate.push_str(" de silva");
        println! ("my name mutated {}", my_name_to_mutate)
    }

    fn some_fun_imu_ref(my_name_to_mutate:  &String){

        let mut a_new_var_to_mutate = String::from("John ");
        a_new_var_to_mutate.push_str(my_name_to_mutate);

        println! ("inside function the variable is mutated {}", a_new_var_to_mutate)
    }


    pub fn test_string_manipulations() {
        let string = String::from("127.0.0.1:8080");

        /* note something special about this string slice .
        i.e. the string_slice is a pointer to the same address location in heap, but it points only
        from 10 to 12 . Note it doesnt create a new location in the heap */
        let string_slice = &string[10..12];
        // note 10 to 12, is counting the 10th byte to the 12th byte.
        // this may differ if you have a charachter with two bytes.🤸  🏽‍♂️🥎

        dbg!(&string_slice);
        println!("string slice should be 80 .. received : {}",string_slice);

        /** borrow the entire string to a string slice **/
        let string_slice_borrowed:&str = &string;
        dbg!(string_slice_borrowed);


        let string_literal = "this is a string slice in stack";
        dbg!(string_literal);

        let string_allocated_in_heap = "string literal in stack".to_string();
        dbg!(string_allocated_in_heap);
    }

    pub fn test_array_and_iteration() {
        let my_numbers: [u32; 5]  = [1,2,3,4,5];
        for i in my_numbers {
            println!("number in array {}",i);
        }
    }
}