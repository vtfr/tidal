use syn::spanned::Spanned;
use syn::{Error, GenericArgument, PathArguments, Type, TypePath};

use crate::evaluator_input::DataType;
use crate::helper;

impl TryFrom<&TypePath> for DataType {
    type Error = Error;

    fn try_from(value: &TypePath) -> Result<Self, Self::Error> {
        if helper::path_ends_with(&value.path, "f32") {
            return Ok(DataType::Scalar);
        }

        if helper::path_ends_with(&value.path, "Vector3") {
            return Ok(DataType::Vector);
        }

        if helper::path_ends_with(&value.path, "CommandList") {
            return Ok(DataType::Command);
        }

        if helper::path_ends_with(&value.path, "Rc") {
            if let Some(ty) = helper::get_first_generic_argument(&value.path) {
                if helper::path_ends_with(&ty.path, "Mesh") {
                    return Ok(DataType::Mesh);
                }
            }
        }

        Err(Error::new(value.span(), "unknown data type"))
    }
}
