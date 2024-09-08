use decktricks::prelude::*;
use decktricks::providers::flatpak::FlatpakProvider;

fn main() {
    let prov = FlatpakProvider::new("net.davidotek.pupgui2");
    prov.runnable().unwrap().run().unwrap();
}

