pub trait Serialisable {
    fn get_id(&self) -> i32;
    fn serialise(&self) -> String;
    fn deserialise(to_deserialise: String) -> Self;
}
