/// Macro that allows you to quickly define a handlebars helper by passing a
/// name and a closure. The closure arguments are mapped to helper parameters
/// one by one. Named argument with default value is also supported and mapped
/// to helper hash.
///
/// # Examples
///
/// ```rust
/// #[macro_use] extern crate handlebars;
/// #[macro_use] extern crate serde_json;
///
/// handlebars_helper!(is_above_10: |x: u64| x > 10);
/// handlebars_helper!(is_above: |x: u64, { compare: u64 = 10 }| x > compare);
///
/// # fn main() {
/// #
/// let mut handlebars = handlebars::Handlebars::new();
/// handlebars.register_helper("is-above-10", Box::new(is_above_10));
/// handlebars.register_helper("is-above", Box::new(is_above));
///
/// let result = handlebars
///     .render_template("{{#if (is-above-10 12)}}great!{{else}}okay{{/if}}", &json!({}))
///     .unwrap();
///  assert_eq!(&result, "great!");
/// let result2 = handlebars
///     .render_template("{{#if (is-above 12 compare=10)}}great!{{else}}okay{{/if}}", &json!({}))
///     .unwrap();
///  assert_eq!(&result2, "great!");
/// # }
/// ```

#[macro_export]
macro_rules! handlebars_helper {
    ($struct_name:ident: |$($name:ident: $tpe:tt),*
                          $($(,)?{$($hash_name:ident: $hash_tpe:tt=$dft_val:literal),*})?|
                            $body:expr ) => {
        #[allow(non_camel_case_types)]
        pub struct $struct_name;

        impl $crate::HelperDef for $struct_name {
            #[allow(unused_assignments)]
            fn call_inner<'reg: 'rc, 'rc>(
                &self,
                h: &$crate::Helper<'reg, 'rc>,
                _: &'reg $crate::Handlebars<'reg>,
                _: &'rc $crate::Context,
                _: &mut $crate::RenderContext<'reg, 'rc>,
            ) -> Result<Option<$crate::ScopedJson<'reg, 'rc>>, $crate::RenderError> {
                let mut param_idx = 0;

                $(
                    let $name = h.param(param_idx)
                        .map(|x| x.value())
                        .ok_or_else(|| $crate::RenderError::new(&format!(
                            "`{}` helper: Couldn't read parameter {}",
                            stringify!($struct_name), stringify!($name),
                        )))
                        .and_then(|x|
                                  handlebars_helper!(@as_json_value x, $tpe)
                                  .ok_or_else(|| $crate::RenderError::new(&format!(
                                      "`{}` helper: Couldn't convert parameter {} to type `{}`. \
                                       It's {:?} as JSON. Got these params: {:?}",
                                      stringify!($struct_name), stringify!($name), stringify!($tpe),
                                      x, h.params(),
                                  )))
                        )?;
                    param_idx += 1;
                )*

                    $(
                        $(
                            let $hash_name = h.hash_get(stringify!($hash_name))
                                .map(|x| x.value())
                                .map(|x|
                                     handlebars_helper!(@as_json_value x, $hash_tpe)
                                     .ok_or_else(|| $crate::RenderError::new(&format!(
                                         "`{}` helper: Couldn't convert hash {} to type `{}`. \
                                          It's {:?} as JSON. Got these hash: {:?}",
                                         stringify!($struct_name), stringify!($hash_name), stringify!($hash_tpe),
                                         x, h.hash(),
                                     )))
                                )
                                .unwrap_or_else(|| Ok($dft_val))?;
                        )*
                    )?

                let result = $body;
                Ok(Some($crate::ScopedJson::Derived($crate::JsonValue::from(result))))
            }
        }
    };

    (@as_json_value $x:ident, object) => { $x.as_object() };
    (@as_json_value $x:ident, array) => { $x.as_array() };
    (@as_json_value $x:ident, str) => { $x.as_str() };
    (@as_json_value $x:ident, i64) => { $x.as_i64() };
    (@as_json_value $x:ident, u64) => { $x.as_u64() };
    (@as_json_value $x:ident, f64) => { $x.as_f64() };
    (@as_json_value $x:ident, bool) => { $x.as_bool() };
    (@as_json_value $x:ident, null) => { $x.as_null() };
    (@as_json_value $x:ident, Json) => { Some($x) };
}

/// This macro is defined if the `logging` feature is set.
///
/// It ignores all logging calls inside the library.
#[cfg(feature = "no_logging")]
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)*) => {};
    ($($arg:tt)*) => {};
}

/// This macro is defined if the `logging` feature is not set.
///
/// It ignores all logging calls inside the library.
#[cfg(feature = "no_logging")]
#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)*) => {};
    ($($arg:tt)*) => {};
}

/// This macro is defined if the `logging` feature is not set.
///
/// It ignores all logging calls inside the library.
#[cfg(feature = "no_logging")]
#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)*) => {};
    ($($arg:tt)*) => {};
}

/// This macro is defined if the `logging` feature is not set.
///
/// It ignores all logging calls inside the library.
#[cfg(feature = "no_logging")]
#[macro_export]
macro_rules! log {
    (target: $target:expr, $($arg:tt)*) => {};
    ($($arg:tt)*) => {};
}

/// This macro is defined if the `logging` feature is not set.
///
/// It ignores all logging calls inside the library.
#[cfg(feature = "no_logging")]
#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)*) => {};
    ($($arg:tt)*) => {};
}

/// This macro is defined if the `logging` feature is not set.
///
/// It ignores all logging calls inside the library.
#[cfg(feature = "no_logging")]
#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)*) => {};
    ($($arg:tt)*) => {};
}
