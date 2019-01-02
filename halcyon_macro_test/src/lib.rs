#[macro_use]
#[allow(unused_imports)]
extern crate halcyon;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let _abc = 123;
        let a = html! {
            <div a="hello.test" b={} c>
                    <div/>
                    <div h="sdf" a={||{test}}>
                        {test}
                    </div>
            </div>
        };
        println!("{}", a);
    }
}
