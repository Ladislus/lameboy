pub trait BinaryPrintable: std::fmt::Display + std::fmt::Binary {}
impl<T: std::fmt::Display + std::fmt::Binary> BinaryPrintable for T {}

pub trait OctalPrintable: std::fmt::Display + std::fmt::Octal {}
impl<T: std::fmt::Display + std::fmt::Octal> OctalPrintable for T {}

pub trait HexPrintable: std::fmt::Display + std::fmt::UpperHex {}
impl<T: std::fmt::Display + std::fmt::UpperHex> HexPrintable for T {}

pub trait Comparable: Eq + Ord {}
impl<T: Eq + Ord> Comparable for T {}

pub trait BitManipulable: Sized + Copy + std::ops::Add<Output = Self> + std::ops::Sub<Output = Self> + std::ops::Not<Output = Self> + std::ops::Shr<Output = Self> + std::ops::Shl<Output = Self> + std::ops::BitAnd<Output = Self> + std::ops::BitOr<Output = Self> + std::ops::BitXor<Output = Self> {}
impl<T: Sized + Copy + std::ops::Add<Output = Self> + std::ops::Sub<Output = Self> + std::ops::Not<Output = Self> + std::ops::Shr<Output = Self> + std::ops::Shl<Output = Self> + std::ops::BitAnd<Output = Self> + std::ops::BitOr<Output = Self> + std::ops::BitXor<Output = Self>> BitManipulable for T {}

pub trait Integer: Sized + Default + Copy + Comparable + BitManipulable + BinaryPrintable + OctalPrintable + HexPrintable {}
impl<T: Sized + Default + Copy + Comparable + BitManipulable + BinaryPrintable + OctalPrintable + HexPrintable> Integer for T {}