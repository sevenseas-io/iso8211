use iso8211::DataDescriptiveFile;

pub fn assert_read(path: &str) -> DataDescriptiveFile {
    match DataDescriptiveFile::read(path) {
        Ok(d) => d,
        Err(e) => panic!(e),
    }
}

#[test]
#[allow(non_snake_case)]
fn read_s_64__2_1_1() {
    let files = [
        "tests/s_64/2_1_1/CATALOG.031",
        "tests/s_64/2_1_1/GB4X0000.000",
        "tests/s_64/2_1_1/GB5X01NE.000",
        "tests/s_64/2_1_1/GB5X01NW.000",
        "tests/s_64/2_1_1/GB5X01SE.000",
        "tests/s_64/2_1_1/GB5X01SW.000",
        "tests/s_64/2_1_1/GB5X02SE.000",
    ];
    for f in &files {
        assert_read(f);
    }
}
