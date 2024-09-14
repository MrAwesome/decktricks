use std::rc::Rc;
use decktricks::prelude::*;

fn main() {
    let prov = Provider::flatpak("net.davidotek.pupgui2", true);
    let _k = Rc::clone(&prov.data);

    println!("Reference count = {}", Rc::strong_count(&prov.data));
    dbg!(prov.is_running().is_ok());

    let _ = prov.is_runnable().and_then(|p| {
        println!("Reference count = {}", Rc::strong_count(&prov.data));
        Ok(p)
    });

    // TODO: unit test this logic, mocking return values
    //let _ = prov.running().is_err().then(|| prov.runnable().and_then(|p| p.run()));

    dbg!(prov.is_running().is_ok());
    println!("Reference count = {}", Rc::strong_count(&prov.data));

}

