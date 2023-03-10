use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(PackageManager)]
pub fn derive_package_manager(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    quote! {
        impl crate::PackageManager for #ident {
            #[inline]
            fn arg(&mut self, s: impl AsRef<::std::ffi::OsStr>) -> &mut Self {
                self.0.arg(s);
                self
            }

            #[inline]
            fn args<I, S>(&mut self, args: I) -> &mut Self
            where
                I: IntoIterator<Item = S>,
                S: AsRef<::std::ffi::OsStr>,
            {
                self.0.args(args);
                self
            }

            #[inline]
            fn execute(&mut self) -> ::std::io::Result<::std::process::ExitStatus> {
                self.0.status()
            }
        }
    }
    .into()
}
