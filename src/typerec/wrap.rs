use crate::typerec::*;

/// Finds potentially recursive variant elements, and wraps them behind an Rc.
pub(in crate::typerec) fn wrap_variant_elements(mods: &mut [Module]) -> HashSet<VariantElement> {
    let infs = inf_enums::inf_size_enums(mods);

    let mut elements = HashSet::new();

    for m in mods {
        for item in &mut m.ast.items {
            if let Item::Enum(it_enum) = item {
                elements.extend(wrap_enum(it_enum, &infs));
            }
        }
    }

    elements
}

fn wrap_enum(it_enum: &mut ItemEnum, infs: &HashSet<String>) -> HashSet<VariantElement> {
    let mut elements = HashSet::new();

    for variant in &mut it_enum.variants {
        let fields: Vec<&mut Field> = match &mut variant.fields {
            Fields::Named(x) => x.named.iter_mut().collect(),
            Fields::Unnamed(x) => x.unnamed.iter_mut().collect(),
            Fields::Unit => Vec::new(),
        };

        for (i, f) in fields.into_iter().enumerate() {
            let Type::Path(tp) = &f.ty else { continue };
            let last_ident = &tp.path.segments.last().unwrap().ident;
            let ty_name_str = format!("{}", last_ident);
            if infs.contains(&ty_name_str) {
                let wrapped_ty = format!("std::rc::Rc<{}>", f.ty.to_token_stream());
                let wrapped_ty = parse_str::<Type>(&wrapped_ty).unwrap();
                f.ty = wrapped_ty;

                let idx = match &f.ident {
                    Some(id) => ElementIdx::Named(format!("{}", id)),
                    None => ElementIdx::Unnamed(i),
                };
                let variant = format!("{}", variant.ident);
                elements.insert(VariantElement { variant, idx });
            }
            
        }
    }

    elements
}
