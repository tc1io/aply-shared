extern crate maud;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate serde_derive;
extern crate ply;

pub mod page;
pub mod web;
pub mod db;
pub mod firestore;
pub mod pubsub;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
