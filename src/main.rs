use decktricks::prelude::*;

fn main() {
    let prov = Provider::flatpak("net.davidotek.pupgui2", true);

    dbg!(prov.running().is_ok());

    // TODO: unit test this logic, mocking return values
    let _ = prov.running().is_err().then(|| prov.runnable().and_then(|p| p.run()));

    dbg!(prov.running().is_ok());

}

