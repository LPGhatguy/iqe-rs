use std::io::{Cursor, BufRead, Lines};
use types::*;

const IQE_HEADER: &'static str = "# Inter-Quake Export";

struct ParseState {
	model: IqeModel,
	current_mesh: Option<usize>,
}

impl ParseState {
	pub fn new() -> ParseState {
		ParseState {
			model: IqeModel::new(),
			current_mesh: None,
		}
	}
}

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

fn match_lines<T: BufRead>(state: &mut ParseState, lines: &mut Lines<T>) -> Result<(), IqeError> {
	for line_result in lines {
		match line_result {
			Ok(line) => {
				try!(match_line(state, line.as_str()));
			},
			Err(err) => {
				return Err(IqeError::IoError(err));
			}
		}
	}

	Ok(())
}

fn match_line(state: &mut ParseState, line: &str) -> Result<(), IqeError> {
	Ok(())
}

fn load_from_lines<T: BufRead>(lines: &mut Lines<T>) -> Result<IqeModel, IqeError> {
	try!(match_header(lines));

	let mut state = ParseState::new();

	try!(match_lines(&mut state, lines));

	Ok(IqeModel {
		meshes: vec![],
	})
}

/// Tries to load an IQE entity from a string slice.
pub fn load_from_str(source: &str) -> Result<IqeModel, IqeError> {
	let mut lines = Cursor::new(source).lines();

	load_from_lines(&mut lines)
}