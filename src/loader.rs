use std::io::{Cursor, BufRead, Lines};
use types::*;

const IQE_HEADER: &'static str = "# Inter-Quake Export";

fn match_header<T: BufRead>(lines: &mut Lines<T>) -> Result<(), IqeError> {
	match lines.next() {
		Some(line_result) => {
			match line_result {
				Ok(first_line) => {
					if !first_line.starts_with(IQE_HEADER) {
						return Err(IqeError::BadHeader);
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

	Ok(())
}

fn load_from_lines<T: BufRead>(lines: &mut Lines<T>) -> Result<IqeEntity, IqeError> {
	try!(match_header(lines));

	for line in lines {
		println!("{:?}", line);
	}

	Ok(IqeEntity {
		meshes: vec![],
	})
}

/// Tries to load an IQE entity from a string slice.
pub fn load_from_str(source: &str) -> Result<IqeEntity, IqeError> {
	let mut lines = Cursor::new(source).lines();

	load_from_lines(&mut lines)
}