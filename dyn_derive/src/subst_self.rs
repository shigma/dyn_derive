fn subst_self_in_type_path(path: &mut syn::Path, repl: &syn::Ident) {
    for segment in &mut path.segments {
        if segment.ident == "Self" {
            segment.ident = repl.clone();
        }
        match &mut segment.arguments {
            syn::PathArguments::AngleBracketed(args) => {
                for arg in &mut args.args {
                    match arg {
                        syn::GenericArgument::Type(ty) => {
                            subst_self(ty, repl);
                        },
                        _ => {},
                    }
                }
            },
            syn::PathArguments::Parenthesized(args) => {
                for ty in &mut args.inputs {
                    subst_self(ty, repl);
                }
                if let syn::ReturnType::Type(_, ty) = &mut args.output {
                    subst_self(ty, repl);
                }
            },
            syn::PathArguments::None => {},
        }
    }
}

pub fn subst_self(ty: &mut syn::Type, repl: &syn::Ident) {
    match ty {
        syn::Type::Path(path) => {
            if let Some(qself) = &mut path.qself {
                subst_self(&mut qself.ty, repl);
            }
            subst_self_in_type_path(&mut path.path, repl);
        },
        syn::Type::Tuple(tuple) => {
            for ty in &mut tuple.elems {
                subst_self(ty, repl);
            }
        },
        syn::Type::Reference(reference) => {
            subst_self(&mut reference.elem, repl);
        },
        syn::Type::Array(array) => {
            subst_self(&mut array.elem, repl);
        },
        syn::Type::Slice(slice) => {
            subst_self(&mut slice.elem, repl);
        },
        syn::Type::Ptr(ptr) => {
            subst_self(&mut ptr.elem, repl);
        },
        syn::Type::Paren(paren) => {
            subst_self(&mut paren.elem, repl);
        },
        syn::Type::Group(group) => {
            subst_self(&mut group.elem, repl);
        },
        syn::Type::ImplTrait(impl_trait) => {
            for bound in &mut impl_trait.bounds {
                match bound {
                    syn::TypeParamBound::Trait(bound) => {
                        subst_self_in_type_path(&mut bound.path, repl)
                    },
                    _ => {},
                }
            }
        },
        syn::Type::TraitObject(trait_object) => {
            for bound in &mut trait_object.bounds {
                match bound {
                    syn::TypeParamBound::Trait(bound) => {
                        subst_self_in_type_path(&mut bound.path, repl)
                    },
                    _ => {},
                }
            }
        },
        syn::Type::BareFn(bare_fn) => {
            for arg in &mut bare_fn.inputs {
                subst_self(&mut arg.ty, repl);
            }
            if let syn::ReturnType::Type(_, ty) = &mut bare_fn.output {
                subst_self(ty, repl);
            }
        },
        _ => {},
    }
}
