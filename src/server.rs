#![allow(unused_variables)]
use std::io::{ Read, Write };
use crate::http::{Request, Response, StatusCode, ParseError};
use std::net::TcpListener;


/* A trait is like an interface.
If it has a function body, its considered a default impl which can be
overriden by structs that impl the trait

The idea behind the Handler Trait is so that the calling client codec calling the Server run method,
will pass in the Handler with custom logic
to handle the Request and Response.
 */
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response ;
    fn handle_bad_request(&mut self, e: ParseError) -> Response {
        println!("failed to parse error {}",e);
        return Response::new(StatusCode::BadRequest, None);
    }

}

pub struct Server {
        ip: String
 }

    impl Server {
        pub fn new(ip: String) -> Self {
            Self {
                ip: ip
            }
        }

        /* in this method, the function will take ownership of the struct and
           when the metho exists the self reference will be destroyed */
        // pub fn run(self){
        //     println!("running server {}", self.ip);
        // }

        // to avoid instance reference, not going out of scope and deallocated
        pub fn run(&self, mut handler: impl Handler) -> Response {
            println!("running server {}", self.ip);
            // without using unwrap, it will return a Result.
            // let listener:Result<TcpListener, E> = TcpListener::bind(&self.ip);

            //if bind was success will return the okay value of the Result
            let listener = TcpListener::bind(&self.ip).unwrap();

            /*

            while true {

            }

            */
            loop {
                    match listener.accept() {
                        Ok((mut tcp_stream, _)) => {
                            let mut buffer = [0; 1024];

                            match tcp_stream.read(&mut buffer) {
                                Ok(_) => {
                                    println!("Received {}", String::from_utf8_lossy(&buffer));

                                    let response: Response  = match Request::try_from(&buffer as &[u8]) {
                                        Ok(request) => handler.handle_request(&request),
                                        Err(e) => handler.handle_bad_request(e),
                                    };

                                    if let Err(e) = response.send(&mut tcp_stream) {
                                        println!("Failed to send response: {}", e);
                                    }

                                }
                                Err(e) => println!("failed to read from connection  {}", e)
                            };


                        }
                        Err(e) => { println! ("failed to establish a conections {}", e);}
                    }
            }

        }

        pub fn run2(&self) {
            println!("running server - run2 {}", self.ip);
        }
    }