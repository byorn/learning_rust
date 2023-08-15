use std::collections::HashMap;
use std::ops::Index;

// a=1&b-2&c&d=&e===&d=7&d=abc

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<& 'buf str, Value<'buf>>
}


/* A heap allocated aray is used which is called a Vector ..Vec */
/* A stack array needs a fixed length at compile time hence we cant use it here as we
are not sure how much element we would need for a multiple
 */
#[derive(Debug)]
pub enum Value<'buf> {
    Single(& 'buf str),
    Multiple(Vec<& 'buf str>)
}

impl<'buf> QueryString<'buf>{
    pub fn get(&self, key: &str) -> Option<&Value> {
        return self.data.get(key);
    }
}

impl<'buf> From<& 'buf str> for QueryString<'buf> {
    // a=1&b-2&c&d=&e===&d=7&d=abc
    fn from(value: &'buf str) -> Self {
         let mut data = HashMap::new();
        for substr in value.split('&') {
            let mut key = substr;
            let mut value = "";
            if let Some(i)= substr.find('='){
                key = &substr[..i];
                value = &substr[i+1..];
            }

            /* Add the enums to the hash map */
            data.entry(key).and_modify(|existing:  &mut Value |
                match existing {
                    Value::Single(prev_value) => {
                        /*
                        vec! is a macro that does
                        let vec = Vec::new()
                        vec.add(prev_value);
                        vec.add(value);
                        */

                        // * means to de-reference the existing pointer, which is
                        // currently pointing to Single.
                        // means replace the memory location with the new value. the reference still points to the same memory location
                        *existing = Value::Multiple(vec!(prev_value, value));

                    }
                    Value::Multiple(vec) => { vec.push(value)}
                }).or_insert(Value::Single(value));

        }
        return Self { data };
    }
}