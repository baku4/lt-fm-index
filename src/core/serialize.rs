pub trait Serialize {
    fn save_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error>;
    fn load_from<R: std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> where
        Self: Sized;
    fn encoded_len(&self) -> usize;
}