#[allow(unused_imports)]
extern crate halcyon;
#[macro_use]
#[allow(unused_imports)]
extern crate halcyon_macro;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let _abc = 123;
        let a = html! {
            <div a="123" b="test" c={}>
                <h1>{}</h1>
            </div>
        };
        println!("{}", a);
    }
}
