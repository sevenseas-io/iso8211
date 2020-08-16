use iso8211::DataDescriptiveFile;

fn main() {
    let ddf = DataDescriptiveFile::read("tests/s_64/2_1_1/GB4X0000.000").unwrap();

    let ddr = ddf.data_descriptive_record();

    println!("\nDDR Leader");
    let ddr_leader = ddr.leader();
    println!("  record_length: {}", ddr_leader.record_length());
    println!(
        "  field_control_length: {}",
        ddr_leader.field_control_length()
    );
    println!("  entry_map:");
    let entry_map = ddr_leader.entry_map();
    println!("    field_length: {}", entry_map.field_length());
    println!("    field_position: {}", entry_map.field_position());
    println!("    field_tag: {}", entry_map.field_tag());
}
