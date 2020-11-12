extern crate maud;

pub mod page;
pub mod web;
pub mod db;
pub mod ply;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
