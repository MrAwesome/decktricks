use decktricks::providers::simple_command::{SimpleCommandProvider,SimpleCommandProviderData,DefaultState};
use decktricks::provider_types::*;
use std::marker::PhantomData;

fn testfunc() {
    let x = SimpleCommandProvider {
        data: SimpleCommandProviderData{},
        state: PhantomData::<DefaultState>,
    };

    let lawl = x.installable();
    match lawl {
        Ok(res) => {
            println!("{:?}", res.data);
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }

}

fn main() {
    testfunc();
    println!("Hello, world!");
}

