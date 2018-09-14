use syn::{Attribute, Error, GenericArgument, Path, PathArguments, Type, TypePath};

#[derive(Default)]
pub struct ErrorAccumulator {
    errors: Vec<Error>,
}

impl ErrorAccumulator {
    pub fn accumulate(&mut self) -> Result<(), Error> {
        let errors = std::mem::replace(&mut self.errors, vec![]);
        let errors = errors.into_iter().reduce(|mut acc, err| {
            acc.combine(err);
            acc
        });

        match errors {
            None => Ok(()),
            Some(err) => Err(err),
        }
    }

    pub fn add(&mut self, err: Error) {
        self.errors.push(err);
    }
}

impl Extend<Error> for ErrorAccumulator {
    fn extend<T: IntoIterator<Item = Error>>(&mut self, iter: T) {
        self.errors.extend(iter.into_iter())
    }
}

impl From<Vec<Error>> for ErrorAccumulator {
    fn from(errors: Vec<Error>) -> Self {
        Self { errors }
    }
}

pub fn path_ends_with(path: &Path, s: &str) -> bool {
    if let Some(path) = path.segments.last() {
        path.ident.eq(s)
    } else {
        false
    }
}

pub fn get_first_generic_argument(path: &Path) -> Option<&TypePath> {
    let path = path.segments.last()?;

    if let PathArguments::AngleBracketed(a) = &path.arguments {
        if let Some(GenericArgument::Type(Type::Path(ref ty))) = a.args.last() {
            return Some(ty);
        };
    };

    None
}

const ATTRIBUTES: &[&str] = &[
    "state",
    "output",
    "input",
    "attribute",
    "context",
    "fallible",
    "default",
];

pub fn has_attribute(attrs: &[Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident(name))
}

pub fn retain_attributes(attr: &Attribute) -> bool {
    for ident in ATTRIBUTES.iter() {
        if attr.path().is_ident(ident) {
            return false;
        }
    }

    return true;
}
