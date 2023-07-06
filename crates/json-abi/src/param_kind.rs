/// A Solidity parameter type.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum ParamKind {
/// Address.
	Address,
	/// Bytes.
    #[default]
	Bytes,
	/// Signed integer.
	Int(usize),
	/// Unsigned integer.
	Uint(usize),
	/// Boolean.
	Bool,
	/// String.
	String,
	/// Array of unknown size.
	Array(Box<ParamKind>),
	/// Vector of bytes with fixed size.
	FixedBytes(usize),
	/// Array with fixed size.
	FixedArray(Box<ParamKind>, usize),
	/// Tuple containing different types
	Tuple(Vec<ParamKind>),
}
