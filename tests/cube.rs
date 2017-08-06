extern crate iqe;

const CUBE: &'static str = include_str!("data/cube.iqe");

#[test]
fn it_loads_cube() {
	let loaded = iqe::load_from_str(CUBE);

	assert!(loaded.is_ok());

	let result = loaded.unwrap();

	assert_eq!(result.meshes.len(), 1);
}