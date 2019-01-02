#[macro_use]
extern crate halcyon;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let a = html!(abs);
        println!("{}",a);
    }
}
