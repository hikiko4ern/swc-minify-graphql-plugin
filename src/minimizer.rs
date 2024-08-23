//! GraphQL minification helpers
//!
//! Logic is built on several axioms:
//! - [`Tpl`] always contains at least one [`TplElement`]
//! - [`Tpl`] always starts with [`TplElement`], which is potentially empty
//! - [`TplElement`] and [`Expr`] always alternate,
//!   i.e. [`Expr`] always follows [`TplElement`] and [`Expr`] always follows [`TplElement`]
//!
//! Since separate parts of GraphQL ([`TplElement`]) and not the whole query ([`Tpl`]) are minified,
//! it is necessary to manually surround [`Expr`] with spaces in some cases,
//! so that identifiers from the expression are not glued with identifiers from the parts
//! after expression substitution in runtime:\
//! `id image{${IMAGE} url}` minified in parts will turn into `id image{${IMAGE}url}`,
//! which in the case where `IMAGE` is an identifier `id` will turn into `id image{idurl}`
//! instead of the correct `id image{id url}`
//!
//! Surrounds with spaces if:
//! - [`Expr`] is followed by [`Expr`], and [`TplElement`] does not end with one of [`Punctuator`]s
//! - the current [`TplElement`] was preceded by [`Expr`], and [`TplElement`] does not start with one of [`Punctuator`]s
//!
//! [`Tpl`]: swc_core::ecma::ast::Tpl
//! [`TplElement`]: swc_core::ecma::ast::TplElement
//! [`Expr`]: swc_core::ecma::ast::Expr
//! [`Punctuator`]: https://spec.graphql.org/October2021/#Punctuator
// spell-checker: ignore idurl

use swc_core::{
    atoms::Atom,
    common::errors::HANDLER,
    ecma::ast::{Str, Tpl},
};

/// [`Punctuator`] characters
///
/// <div class="warning">
///
/// punctuator `...` is not checked for a complete match --- any `.` is considered as a part of `...`,
/// since the only [`Token`] whose beginning or end is `.` is `...`
///
/// cases where [`Expr`] breaks [`Token`] (e.g. `some${LONG}FieldName`, `123.${FP}` or `"some. ${STR} string"`)
/// are considered invalid and are not handled properly
///
/// </div>
///
/// [`Punctuator`]: https://spec.graphql.org/October2021/#Punctuator
/// [`Token`]: https://spec.graphql.org/October2021/#Token
/// [`Expr`]: swc_core::ecma::ast::Expr
const PUNCTUATORS: &[char] = &[
    '!', '$', '&', '(', ')', '.', ':', '@', '[', ']', '{', ',', '}',
];

/// minifies [`Str`]
pub fn minify_graphql_str(str: &mut Str) {
    let value = str.value.as_str();

    if !value.is_empty() {
        match graphql_minify::minify(value) {
            Ok(min) => {
                str.value = Atom::new(min);
                str.raw = None;
            }
            Err(err) => HANDLER.with(|handler| {
                handler
                    .struct_span_err(str.span, &format!("failed to minify GraphQL: {err:#?}"))
                    .emit();
            }),
        }
    }
}

/// minifies [`Tpl`]
pub fn minify_graphql_tpl(tpl: &mut Tpl) {
    if tpl.exprs.is_empty() {
        // If there are no expressions, we take the shortest path and
        // minify the single `TplElement` without additional checks

        let tpl_el = unsafe { tpl.quasis.get_unchecked_mut(0) };
        let query = tpl_el.cooked.as_ref().unwrap_or(&tpl_el.raw).as_str();

        if !query.is_empty() {
            match graphql_minify::minify(query) {
                Ok(min) => {
                    tpl_el.raw = Atom::new(min);
                    tpl_el.cooked = Some(tpl_el.raw.clone());
                }
                Err(err) => HANDLER.with(|handler| {
                    handler
                        .struct_span_err(
                            tpl_el.span,
                            &format!("failed to minify GraphQL: {err:#?}"),
                        )
                        .emit();
                }),
            }
        }

        return;
    }

    // minify all `TplElement`s, surrounding expressions with spaces if necessary

    let mut expr_it = tpl.exprs.iter();
    let mut has_prev_expr = false;
    let last_quasis_index = tpl.quasis.len() - 1;

    for (i, tpl_el) in tpl.quasis.iter_mut().enumerate() {
        let query = tpl_el.cooked.as_ref().unwrap_or(&tpl_el.raw).as_str();
        let next_is_expr = expr_it.next().is_some();

        if !query.is_empty() {
            match graphql_minify::minify(query) {
                Ok(mut min) => {
                    let is_empty = min.is_empty();
                    let mut is_space_inserted = false;

                    if has_prev_expr
                        && !(is_empty && last_quasis_index == i)
                        && !min.starts_with(PUNCTUATORS)
                    {
                        min.insert(0, ' ');
                        is_space_inserted = true;
                    }

                    if next_is_expr
                        && !(is_empty && (is_space_inserted || i == 0))
                        && !min.ends_with(PUNCTUATORS)
                    {
                        min.push(' ');
                    }

                    tpl_el.raw = Atom::new(min);
                    tpl_el.cooked = Some(tpl_el.raw.clone());
                }
                Err(err) => HANDLER.with(|handler| {
                    handler
                        .struct_span_err(
                            tpl_el.span,
                            &format!("failed to minify GraphQL: {err:#?}"),
                        )
                        .emit();
                }),
            }
        }

        has_prev_expr = next_is_expr;
    }
}
