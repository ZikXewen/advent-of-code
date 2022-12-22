use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Creates a `main` function executing the `compute` function in your current file against `input.txt`.
///
/// If called with a string, this will also create a `test` function `compute`ing againt `test.txt`
///
/// DEPRECATED: Use `#[aoc::main]` instead
#[proc_macro]
pub fn make_main(item: TokenStream) -> TokenStream {
    let item = item.to_string();

    let main = quote! {
        fn main() {
            let input = std::fs::read_to_string("./input.txt").expect("Error reading input.txt");
            println!("{}", compute(input));
        }
    };

    let test = quote! {
        #[cfg(test)]
        #[test]
        fn test() {
            let input = std::fs::read_to_string("./test.txt").expect("Error reading test input");
            assert_eq!(compute(input).to_string(), #item);
        }
    };

    if item.len() > 0 {
        quote! {
            #main
            #test
        }
        .into()
    } else {
        main.into()
    }
}

/// Creates a `main` function executing this function against `input.txt`.
///
/// This function is automatically assign an `input` parameter of type `String`,
/// while expected return value implements `Display + ToString`.
///
/// If called with an argument, this will also create a `test` function executing againt `test.txt`
/// with that argument as the expected answer.
///
/// # Example
///
/// ```rust
/// #[aoc::main(answer)]
/// fn main() {
///    "answer"
/// }
/// ```
#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let block = parse_macro_input!(item as ItemFn).block;
    let main_quote = quote! {
        fn compute(input: String) -> impl std::fmt::Display + std::string::ToString
            #block
        fn main() {
            let input = std::fs::read_to_string("./input.txt").expect("Error reading input.txt");
            println!("{}", compute(input));
        }
    };

    if attr.is_empty() {
        return main_quote.into();
    }

    let attr = attr.to_string();
    let test_quote = quote! {
        #[cfg(test)]
        #[test]
        fn test() {
            let input = std::fs::read_to_string("./test.txt").expect("Error reading test input");
            assert_eq!(compute(input).to_string(), #attr);
        }
    };

    quote! {
        #main_quote
        #test_quote
    }
    .into()
}
