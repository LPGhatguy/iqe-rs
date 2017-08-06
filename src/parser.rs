use std::io::{Cursor, BufRead, Lines};
use std::str::{Chars};
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

trait Taker {
	fn take<'a>(&self, source: &'a str) -> Option<&'a str>;
}

struct TakeSequence {
	takers: Vec<Box<Taker>>,
}

impl Taker for TakeSequence {
	fn take<'a>(&self, source: &'a str) -> Option<&'a str> {
		let mut current_source = source;

		for taker in &self.takers {
			match taker.take(current_source) {
				Some(new_source) => {
					current_source = new_source;
				},
				None => {
					return None;
				}
			}
		}

		Some(current_source)
	}
}

impl TakeSequence {
	pub fn new(takers: Vec<Box<Taker>>) -> TakeSequence {
		TakeSequence {
			takers,
		}
	}
}

struct TakeLiteral<'a> {
	literal: &'a str,
}

impl<'a> Taker for TakeLiteral<'a> {
	fn take<'b>(&self, source: &'b str) -> Option<&'b str> {
		let mut zip = source.char_indices().zip(self.literal.chars());

		for ((source_pos, source_char), literal_char) in zip {
			if source_char != literal_char {
				return None;
			}
		}

		Some(&source[self.literal.len()..])
	}
}

impl<'a> TakeLiteral<'a> {
	pub fn new(literal: &'a str) -> TakeLiteral {
		TakeLiteral {
			literal,
		}
	}
}

struct TakeInsideBalanced {
	start: char,
	end: char,
}

impl Taker for TakeInsideBalanced {
	fn take<'a>(&self, source: &'a str) -> Option<&'a str> {
		let mut start_pos = 0;
		let mut end_pos = 0;

		let mut iter = source.char_indices();

		if let Some((_, first_char)) = iter.next() {
			if first_char != self.start {
				return None;
			}

			start_pos = first_char.len_utf8();
		} else {
			return None;
		}

		for (pos, char) in iter {
			end_pos = pos;

			if char == self.end {
				break;
			}
		}

		if end_pos == 0 {
			return None;
		}

		return Some(&source[start_pos..end_pos]);
	}
}

impl TakeInsideBalanced {
	pub fn new(start: char, end: char) -> TakeInsideBalanced {
		TakeInsideBalanced {
			start,
			end,
		}
	}
}

struct TakeWhitespace;

impl Taker for TakeWhitespace {
	fn take<'a>(&self, source: &'a str) -> Option<&'a str> {
		let mut start_pos = 0;

		for (pos, char) in source.char_indices() {
			match char {
				' ' | '\t' | '\r' | '\n' => {},
				_ => {
					start_pos = pos;
					break;
				}
			}
		}

		return Some(&source[start_pos..]);
	}
}

impl TakeWhitespace {
	pub fn new() -> TakeWhitespace {
		TakeWhitespace {}
	}
}

struct TakeMaybe<'a> {
	next: &'a Taker,
}

impl<'a> Taker for TakeMaybe<'a> {
	fn take<'b>(&self, source: &'b str) -> Option<&'b str> {
		match self.next.take(source) {
			v@Some(_) => v,
			None => Some(source),
		}
	}
}

impl<'a> TakeMaybe<'a> {
	pub fn new(next: &'a Taker) -> TakeMaybe<'a> {
		TakeMaybe {
			next,
		}
	}
}

struct TakeOr {
	next: Box<Taker>,
	or: &'static str,
}

impl Taker for TakeOr {
	fn take<'b>(&self, source: &'b str) -> Option<&'b str> {
		match self.next.take(source) {
			v@Some(_) => v,
			None => Some(&self.or),
		}
	}
}

impl TakeOr {
	pub fn new(next: Box<Taker>, or: &'static str) -> TakeOr {
		TakeOr {
			next,
			or,
		}
	}
}

/// Matches the header of an IQE file.
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

/// Matches some lines of an IQE file.
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

/// Matches one line of an IQE file.
fn match_line(state: &mut ParseState, line: &str) -> Result<(), IqeError> {
	let first_entry = line.split_whitespace().nth(0);

	if let Some(command) = first_entry {
		match command {
			"mesh" => match_mesh(state, line.trim()),
			"#" => Ok(()),
			// _ => Err(IqeError::InvalidData),
			_ => Ok(()),
		}
	} else {
		Ok(())
	}
}

/// Matches `mesh` or `mesh "NAME"`
fn match_mesh(state: &mut ParseState, source: &str) -> Result<(), IqeError> {
	let taker = TakeSequence::new(vec![
		Box::new(TakeLiteral::new("mesh")),
		Box::new(TakeWhitespace::new()),
		Box::new(TakeOr::new(
			Box::new(TakeInsideBalanced::new('"', '"')),
			"",
		)),
	]);

	match taker.take(source) {
		Some(source) => {
			let mesh = IqeMesh::new(source);
			state.model.meshes.push(mesh);
		},
		None => {
			return Err(IqeError::InternalParserError);
		},
	}

	Ok(())
}

fn load_from_lines<T: BufRead>(lines: &mut Lines<T>) -> Result<IqeModel, IqeError> {
	try!(match_header(lines));

	let mut state = ParseState::new();

	try!(match_lines(&mut state, lines));

	Ok(state.model)
}

/// Tries to load an IQE entity from a string slice.
pub fn load_from_str(source: &str) -> Result<IqeModel, IqeError> {
	let mut lines = Cursor::new(source).lines();

	load_from_lines(&mut lines)
}