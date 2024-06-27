pub fn is_dyn(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::TraitObject(_) => true,
        syn::Type::Reference(r) => is_dyn(&r.elem),
        syn::Type::Path(p) => {
            for seg in &p.path.segments {
                let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
                    continue
                };
                for arg in &args.args {
                    let syn::GenericArgument::Type(ty) = arg else {
                        continue
                    };
                    if is_dyn(ty) {
                        return true
                    }
                }
            }
            false
        },
        _ => false,
    }
}
