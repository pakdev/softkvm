use ddc_hi::{Ddc, Display};

fn main() {
    for mut display in Display::enumerate() {
        display.update_capabilities().unwrap();
        println!(
            "{:?} {}: {:?} {:?}",
            display.info.backend,
            display.info.id,
            display.info.manufacturer_id,
            display.info.model_name
        );

        match display.info.manufacturer_id.as_deref() {
            Some("ACR") => display.handle.set_vcp_feature(0x60, 15).unwrap(),
            _ => println!("Not here"),
        }

        // if let Some(feature) = display.info.mccs_database.get(0xdf) {
        //     let value = display.handle.get_vcp_feature(feature.code).unwrap();
        //     println!("{}: {:?}", feature.name.as_ref().unwrap(), value);
        // }
    }
}
