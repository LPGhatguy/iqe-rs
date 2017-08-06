extern crate iqe;

const IQE_HEADER: &'static str = "# Inter-Quake Export";

#[test]
fn it_fails_bad_data() {
	let bad_data = "the lies of the jedi";

	let result = iqe::load_from_str(&bad_data);

	assert!(result.is_err());
}

#[test]
fn it_loads_empty_data() {
	let result = iqe::load_from_str(&IQE_HEADER);

	println!("{:?}", result);

	assert!(result.is_ok());
}