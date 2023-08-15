use std::fmt::{Display, Formatter};
use std::net::TcpStream;
use std::io::{ Write, Result as IOResult};
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>)->Response {
        Self {
            status_code, body
        }
    }


    // writing directly to the stream instead of the formatter for efficient memory usage
    // i.e. write! macro called directly to the stream. as oposed to the code write below,
    // which implements the display trait.

    //notice: note the impl keywork. Write is a train, or interface, that would expect
    // any concrete type like 'File' or 'TcpStream'
    // when we use the impl keywork, we tell the compiler to look at the calling client code, and
    // generate the assemply code with the correct types.
    // we can also use "dyn Write", where dyn means dynamic, but this mean there is more overhead
    // in the assembly code to figure out the concreete code to use.
    pub fn send(&self, stream: &mut impl Write) -> IOResult<()>{

        let body = match &self.body {
            None => { ""}
            Some(someVal) => {someVal}
        };


        write!(stream, "HTTP/1.1 {} {} \r\n\r\n{}",
               self.status_code,
               self.status_code.reason_phrase(),
               body
        )
    }

}

/*

if you want to pass the Respone into the write! macro as a parameter,
then you have to implement the Display Traits interface

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let body = match &self.body {
            None => { ""}
            Some(someVal) => {someVal}
        };
        // using the f formatter here, means the values you pass in to this
        // will all be in memory.
        write!(f, "HTTP/1.1 {} {} \r\n\r\n{}",
        self.status_code,
        self.status_code.reason_phrase(),
        body
        )
    }
}

 */