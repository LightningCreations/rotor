#![deny(warnings)]

pub mod cog;
pub mod hub;
pub mod p2;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
