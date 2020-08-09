mod common;
use common::assert_read;

#[test]
#[allow(non_snake_case)]
fn CATALOG() {
    let _target = assert_read("tests/s_64/2_1_1/CATALOG.031");
}

#[test]
#[allow(non_snake_case)]
fn GB4X0000() {
    let _target = assert_read("tests/s_64/2_1_1/GB4X0000.000");
}

#[test]
#[allow(non_snake_case)]
fn GB5X01NE() {
    let _target = assert_read("tests/s_64/2_1_1/GB5X01NE.000");
}

#[test]
#[allow(non_snake_case)]
fn GB5X01NW() {
    let _target = assert_read("tests/s_64/2_1_1/GB5X01NW.000");
}

#[test]
#[allow(non_snake_case)]
fn GB5X01SE() {
    let _target = assert_read("tests/s_64/2_1_1/GB5X01SE.000");
}

#[test]
#[allow(non_snake_case)]
fn GB5X01SW() {
    let _target = assert_read("tests/s_64/2_1_1/GB5X01SW.000");
}

#[test]
#[allow(non_snake_case)]
fn GB5X02SE() {
    let _target = assert_read("tests/s_64/2_1_1/GB5X02SE.000");
}
