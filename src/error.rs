#[derive(Fail, Debug)]
#[fail(display = "Library not found: {}", name)]
pub struct LibraryNotFoundError {
    pub name: String
}
