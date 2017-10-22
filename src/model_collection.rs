/// collection of structs

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageCommand<'a> {
    pub command: &'a str,
    pub parameter: &'a str,
}

