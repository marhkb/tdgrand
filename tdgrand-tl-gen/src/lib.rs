// Copyright 2021 - developers of the `tdgrand` project.
// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module gathers all the code generation submodules and coordinates
//! them, feeding them the right data.
mod enums;
mod functions;
mod grouper;
mod metadata;
mod rustifier;
mod types;

use std::io::{self, Write};
use tdgrand_tl_parser::tl::{Definition, Type};

/// Don't generate types for definitions of this type,
/// since they are "core" types and treated differently.
const SPECIAL_CASED_TYPES: [&str; 5] = ["Bool", "Bytes", "Int32", "Int53", "Int64"];

fn ignore_type(ty: &Type) -> bool {
    SPECIAL_CASED_TYPES.iter().any(|&x| x == ty.name)
}

pub fn generate_rust_code(file: &mut impl Write, definitions: &[Definition]) -> io::Result<()> {
    write!(
        file,
        "\
         // Copyright 2021 - developers of the `tdgrand` project.\n\
         // Copyright 2020 - developers of the `grammers` project.\n\
         //\n\
         // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or\n\
         // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license\n\
         // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your\n\
         // option. This file may not be copied, modified, or distributed\n\
         // except according to those terms.\n\
         "
    )?;

    let metadata = metadata::Metadata::new(&definitions);
    types::write_types_mod(file, definitions, &metadata)?;
    enums::write_enums_mod(file, definitions, &metadata)?;
    functions::write_functions_mod(file, definitions, &metadata)?;

    Ok(())
}
