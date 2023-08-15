#![allow(dead_code)]

mod test;
mod server;
mod http;

mod website_handler;

use server::Server;
use test::Playground;
use website_handler::WebsiteHandler;
use std::env;

fn main() {

    let pg = Playground::new();
    pg.test_playground();

    /*
    Note: i wasnt able to do
    pg.test_playground2(), because in the above function, the pg variable pionter gets passed into the
    the test_playground and gets deallocated. the value is already moved.
    hence i cant call pg.test_playground(2) this way again.
     */
    Playground::test_playground2();
    Playground::test_playground3();
    Playground::test_any_immutable_ref_or_single_mutable_reference();
    Playground::test_array_and_iteration();
    /* note in the run method, like above the self doesnt get deallocated */
    let my_server = Server::new(String::from("localhost:8080"));

    let default_path = format!("{}/public",env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path is {}", public_path);
    let website_handler = WebsiteHandler::new(public_path);

    my_server.run(website_handler);


    Playground::test_string_manipulations();
}

fn check_reference1(my_variable: &mut String){
    println!("check reference 1 method {}", my_variable);
}

fn check_reference2(my_variable: &mut String){
    println!("check reference 2 method {}", my_variable);
}


fn calculate_my_first_calculation(my_number: u32) -> u32 {

    let my_new_number = my_number * 100;
    return my_new_number;
}
