use std::io::{self, Cursor, BufRead};

const IQE_HEADER: &'static str = "# Inter-Quake Export";

#[derive(Debug)]
pub struct IqeMesh {
	name: String,
	positions: Option<Vec<[f32; 4]>>,
	texture_coords: Option<Vec<[f32; 2]>>,
	normals: Option<Vec<[f32; 3]>>,
}

#[derive(Debug)]
pub struct IqeResult {
	meshes: Vec<IqeMesh>,
}

#[derive(Debug)]
pub enum IqeError {
	Empty,
	NoHeader,
	IoError(io::Error),
	InvalidData,
}

pub fn load_from_str(source: &str) -> Result<IqeResult, IqeError> {
	let mut cursor = Cursor::new(source);
	let mut lines = cursor.lines();

	match lines.next() {
		Some(line_result) => {
			match line_result {
				Ok(first_line) => {
					if !first_line.starts_with(IQE_HEADER) {
						return Err(IqeError::NoHeader);
					}
				},
				Err(err) => {
					return Err(IqeError::IoError(err));
				},
			}
		},
		None => {
			return Err(IqeError::Empty);
		},
	}

	for line in lines {
		println!("{:?}", line);
	}

	Ok(IqeResult {
		meshes: vec![],
	})
}