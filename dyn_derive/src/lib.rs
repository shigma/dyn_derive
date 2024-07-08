#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

mod dyn_trait;
mod generics;
mod subst_self;

/// This is a procedural macro for deriving object-unsafe traits.
/// 
/// ## Example
/// 
/// `Clone` is not object-safe, but with this macro, you can still use `dyn Meta`:
/// 
/// ```
/// use dyn_derive::*;
/// 
/// #[dyn_trait]
/// pub trait Meta: Clone {}
/// 
/// #[derive(Clone)]
/// pub struct Foo {
///     meta: Box<dyn Meta>,
/// }
/// ```
#[proc_macro_attribute]
pub fn dyn_trait(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse2(input.into()).expect("expect trait");
    dyn_trait::transform(attr.into(), item).into()
}

#[cfg(test)]
mod test {
    use std::env::args_os;
    use std::ffi::OsString;
    use std::fs::{create_dir_all, read_to_string, write};
    use std::mem::replace;
    use std::path::{Path, PathBuf};

    use pretty_assertions::StrComparison;
    use prettyplease::unparse;
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};
    use walkdir::WalkDir;

    use crate::dyn_trait::transform;

    fn transform_input(input: TokenStream) -> TokenStream {
        let mut item: syn::ItemTrait = syn::parse2(input).unwrap();
        let attrs = replace(&mut item.attrs, vec![]);
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs[0].path().to_token_stream().to_string(), "dyn_trait".to_string());
        let attr = match &attrs[0].meta {
            syn::Meta::Path(_) => quote! {},
            syn::Meta::List(list) => list.tokens.clone(),
            syn::Meta::NameValue(_) => unimplemented!(),
        };
        transform(attr, item)
    }

    struct TestDiff {
        path: PathBuf,
        expect: String,
        actual: String,
    }

    #[test]
    fn fixtures() {
        let args = args_os().collect::<Vec<_>>();
        let input_dir = "fixtures/input";
        let output_dir = "fixtures/output";
        let mut diffs = vec![];
        let will_emit = args.contains(&OsString::from("emit"));
        for entry in WalkDir::new(input_dir).into_iter().filter_map(Result::ok) {
            let input_path = entry.path();
            if !input_path.is_file() || input_path.extension() != Some("rs".as_ref()) {
                continue;
            }
            let path = input_path.strip_prefix(input_dir).unwrap();
            let output_path = Path::new(output_dir).join(path);
            let input = read_to_string(input_path).unwrap().parse().unwrap();
            let actual = unparse(&syn::parse2(transform_input(input)).unwrap());
            let expect_result = read_to_string(&output_path);
            if let Ok(expect) = &expect_result {
                if expect == &actual {
                    continue;
                }
            }
            if will_emit {
                create_dir_all(output_path.parent().unwrap()).unwrap();
                write(output_path, &actual).unwrap();
            }
            if let Ok(expect) = expect_result {
                diffs.push(TestDiff {
                    path: path.to_path_buf(),
                    expect,
                    actual,
                });
            }
        }
        let len = diffs.len();
        for diff in diffs {
            eprintln!("diff {}", diff.path.display());
            eprintln!("{}", StrComparison::new(&diff.expect, &diff.actual));
        }
        if len > 0 && !will_emit {
            panic!("Some tests failed");
        }
    }
}
