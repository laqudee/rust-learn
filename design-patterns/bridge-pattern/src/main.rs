mod device;
mod remotes;

use device::{Device, Radio, TV};
use remotes::{AdvancedRemove, BasicRemote, HasMutableDevice, Remote};

fn main() {
    test_device(TV::default());
    test_device(Radio::default());
}

fn test_device(device: impl Device + Clone) {
    println!("Tests with basic remote.");
    let mut basic_remote = BasicRemote::new(device.clone());
    basic_remote.power();
    basic_remote.device().print_status();

    println!("Tests with advanced remote.");
    let mut advance_remote = AdvancedRemove::new(device);
    advance_remote.power();
    advance_remote.mute();
    advance_remote.device().print_status();
}
