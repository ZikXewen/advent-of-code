use proc_macro::TokenStream;
use quote::quote;

/// Creates a `main` function executing the `compute` function in your current file against `input.txt`.
///
/// If called with a string, this will also create a `test` function `compute`ing againt `test.txt`
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
