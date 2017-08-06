use std::io;

/// Represents one mesh loaded from an IQE file.
///
/// All `Option<Vec<T>>` members of this struct, if present, must all be the
/// same length. This is defined by the IQE specification.
#[derive(Debug)]
pub struct IqeMesh {
	/// The friendly name of the mesh.
	pub name: String,

	/// The vertex positions in the mesh, if any.
	pub positions: Option<Vec<[f32; 4]>>,

	/// The texture coordinates in the mesh, if any.
	pub texture_coords: Option<Vec<[f32; 2]>>,

	/// The normals in the mesh, if any.
	pub normals: Option<Vec<[f32; 3]>>,

	/// The faces in the mesh, if any.
	///
	/// Faces are defined as a set of 3 indices into the other buffers on the
	/// mesh. These indices are zero-based.
	pub faces: Option<Vec<[usize; 3]>>,
}

impl IqeMesh {
	/// Creates a new, empty IqeMesh with the given name.
	pub fn new(name: &str) -> IqeMesh {
		IqeMesh {
			name: String::from(name),
			positions: None,
			texture_coords: None,
			normals: None,
			faces: None,
		}
	}
}

/// Represents the result of loading an IQE file.
///
/// An entity contains zero or more named [IqeMesh](struct.IqeMesh.html) objects.
#[derive(Debug)]
pub struct IqeModel {
	pub meshes: Vec<IqeMesh>,
}

impl IqeModel {
	/// Creates a new, empty IqeModel.
	pub fn new() -> IqeModel {
		IqeModel {
			meshes: Vec::new(),
		}
	}
}

/// Possible error conditions reachable when loading an IQE file.
#[derive(Debug)]
pub enum IqeError {
	/// The data passed to the loader was empty.
	Empty,

	/// The file passed to the loader did not have a valid IQE header.
	BadHeader,

	/// There was an IO error. This is propagated from the `Lines` iterator,
	/// presently.
	IoError(io::Error),

	/// There was some invalid data present in the IQE file somewhere after the
	/// header.
	InvalidData,

	/// Something went wrong with the parser internally.
	InternalParserError,
}