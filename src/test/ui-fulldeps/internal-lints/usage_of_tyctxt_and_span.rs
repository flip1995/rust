// compile-flags: -Z unstable-options

#![feature(rustc_private)]
#![deny(usage_of_tyctxt_and_span_args)]
#![allow(unused)]

extern crate rustc;
extern crate syntax_pos;

use rustc::ty::{TyCtxt, query::TyCtxtAt};
use syntax_pos::Span;

fn both(tcx: TyCtxt, span: Span) {} //~ ERROR using both `TyCtxt` and `Span` as arguments

fn at(tcx_at: TyCtxtAt) {}

fn main() {}
