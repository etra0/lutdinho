use lutdinho::cube::Cube;

static CUBE_FILE: &str = std::concat!(std::env!("CARGO_MANIFEST_DIR"), "/tests/test_lut.cube");

#[test]
fn test_parse() {
    let cube = Cube::parse(CUBE_FILE).unwrap();
    assert_eq!(cube.size, 32);
    assert_eq!(cube.values.len(), 32768);
}

#[test]
fn test_image() {
    let cube = Cube::parse(CUBE_FILE).unwrap();
    let img = cube.generate_image(Some(64)).unwrap();
    img.save("output.png").unwrap();
}
