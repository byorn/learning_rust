use super::Method;
use super::{QueryString};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str;
use std::str::Utf8Error;
use crate::http::method::MethodError;

/**
if you are using references as properties in a struct, you have to specify the lifetime of the referfence.
Where it points to in memory ?
**/
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    method: Method,

    /* This means if the value is null Option will be none */
    query_string: Option<QueryString<'buf>>
}


// inorder for the calling client code to use the Request properties, we have to expose getters
// for the Request
impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> &Option<QueryString> {
        &self.query_string
    }
}

/** 'buf is to indicate to compiler that all the references marked with the lifetime
will live until the scope it was defined, that all marked references share the same lifetime in memory */
impl<'buf> TryFrom<&'buf[u8]> for Request<'buf> {
    type Error = ParseError;

    //example Request string : GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidProtocol)?;
        let mut query_string = None;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        //1. extracting the path

        // let qOption = path.find('?');
        // match qOption {
        //     None => {}
        //     Some(i) => {
        //
        //         query_string = Some(&path[i+1..]);
        //         path = &path[..i];
        //     }
        // }

        //2. way of extracting the path

        // let mut q = path.find('?');
        // if (q.is_some()){
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }

        //3 rd way advanced way of extracting path
        if let Some(i) = path.find('?') {
                query_string = Some(QueryString::from(&path[i + 1..]));
                path = &path[..i];
        }

        return Ok(Self {
            path ,
            method,
            query_string,
        });
    }
}

/**
return a tuple
1) the word that we want
2) the balance string
 **/
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    /*
    let mut iter = request.chars();

    loop {
        let item = iter.next();
        match item {
            None => {}
            Some(_) => {}
        }
    }*/

    for (i,c) in request.chars().enumerate() {
            if c == ' ' || c == '\r'{
                return Some((&request[..i], &request[i+1..]));
            }
    }
    unimplemented!()
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
impl ParseError {
    fn message(&self) -> &str{
            match self {
                ParseError::InvalidRequest => "InvalidRequest",
                ParseError::InvalidEncoding => "InvalidEncoding",
                ParseError::InvalidProtocol => "InvalidProtocol",
                ParseError::InvalidMethod => "InvalidMethod"            }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}

