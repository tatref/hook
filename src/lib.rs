//! The [`hook!`] macro helps to create libraries to do the `LD_PRELOAD` trick
//!
//! <http://www.goldsborough.me/c/low-level/kernel/2016/08/29/16-48-53-the_-ld_preload-_trick/>

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};

/// The macro
///
/// <https://www.netspi.com/blog/technical/network-penetration-testing/function-hooking-part-i-hooking-shared-library-function-calls-in-linux/>
///
/// We'll make a demo with `puts`, whose function signature is ```int puts(const char *s);```
///
/// `puts` will be generated, `fake_puts` must be created manually and match a special signature
/// where the 1st arg is the signature of `puts`, and the remaining args will be the usual args
/// from the `puts` function.
/// The return type must match the return type of `puts`.
///
/// ```no_run
/// use libc::*;
/// use hooked::hook;
///
/// pub unsafe fn fake_puts(
///     real: unsafe extern fn(*const c_char) -> c_int,  // first comes the function signature
///     arg0: *const c_char                              // 1st argument
///     //arg1: int c_char                               // 2nd argument
///     ) -> c_int {                                     // return type
///
///     // just forward the call to the real function
///     real(arg0)
/// }
///
/// hook!(puts, fake_puts, fn(*const c_char) -> c_int);
/// ```
///
///
#[proc_macro]
pub fn hook(items: TokenStream) -> TokenStream {
    // convert to proc_macro2::TokenStream
    let items: proc_macro2::TokenStream = items.into();
    let mut iter = items.into_iter();

    let function_name = iter.next().unwrap();

    let mut iter = iter.skip(1);
    let fake = iter.next().unwrap();

    let iter = iter.skip(1);
    let f: proc_macro2::TokenStream = iter.collect();

    let f: syn::Type = syn::parse2(f).unwrap();
    let f = match f {
        syn::Type::BareFn(f) => f,
        _ => panic!("3rd arg must be a function signature"),
    };

    let mut arguments_def = f.inputs.clone();
    let mut arguments_names = Vec::new();
    for (idx, mut arg) in arguments_def.iter_mut().enumerate() {
        let arg_name = format!("arg{}", idx);
        arguments_names.push(arg_name.clone());
        arg.name = Some((
            syn::Ident::new(&arg_name, Span::call_site()),
            syn::token::Colon {
                spans: [Span::call_site()],
            },
        ));
    }
    let return_type = f.output.to_token_stream();

    let arguments: String = arguments_names.as_slice().join(", ");
    let arguments: proc_macro2::TokenStream = syn::parse_str(&arguments).unwrap();

    let generated = quote!(
        #[no_mangle]
        pub unsafe extern "C" fn #function_name(#arguments_def) #return_type {
            let result = std::panic::catch_unwind(|| {
                use std::ffi::CString;
                use libc::c_void;

                let handle: *mut c_void = libc::RTLD_NEXT;
                let symbol: CString = CString::new(stringify!(#function_name)).unwrap();

                let real = libc::dlsym(handle, symbol.as_ptr());
                if real.is_null() {
                    panic!("Symbol not found: {:?}", symbol.to_string_lossy());
                }

                let real: unsafe extern "C" #f = std::mem::transmute(real);

                #fake(real, #arguments)
            });

            match result {
                Ok(x) => x,
                Err(_) => std::process::abort(),
            }
        }
    );

    // Convert back to proc_macro::TokenStream
    generated.into()
}
