extern crate maud;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate serde_derive;

pub mod page;
pub mod web;
pub mod pubsub;
pub mod datastore;
pub mod problem;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
