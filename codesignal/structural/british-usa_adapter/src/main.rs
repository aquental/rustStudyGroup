mod adapter;

use adapter::{BritishPlug, BritishToUSAdapter, USPlug};

fn main() {
    // Creating an instance of BritishPlug
    let british_plug = BritishPlug;

    // TODO: Create an instance of BritishToUSAdapter using the BritishPlug instance
    let adapter = BritishToUSAdapter::new(british_plug);

    // TODO: Call the plug_in method on the adapter
    adapter.plug_in();
}
