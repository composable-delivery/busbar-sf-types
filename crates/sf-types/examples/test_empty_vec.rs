use busbar_sf_types::metadata::apex::ApexTestSuite;
use busbar_sf_types::traits::XmlSerializable;

fn main() {
    let test_suite = ApexTestSuite {
        test_class_name: vec![],
    };

    let xml = test_suite.to_metadata_xml().expect("Failed to serialize");
    println!("Generated XML:");
    println!("{}", xml);
}
