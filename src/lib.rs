extern crate maud;

pub mod page;
pub mod web;
pub mod db;

extern crate serde;
extern crate serde_json;
//#[macro_use]
extern crate serde_derive;
extern crate ply;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
