#[derive(Fail, Debug)]
#[fail(display = "Cannot resolve name {}", name)]
pub struct CannotResolveError {
    pub name: String
}
