use rustc_lint::{LintStore, LateContext};
use rustc_middle::ty::TyCtxt;

use linter_api::{
    ast::{
        ty::{Ty, TyKind},
        Ident, Lifetime, Span, Symbol,
    },
    context::DriverContext, lint::Lint,
};

use super::{ty::RustcTy, RustcLifetime, RustcSpan};

#[expect(unused)]
pub struct RustcContext<'ast, 'tcx> {
    pub(crate) lint_ctx: LateLint
    pub(crate) tcx: TyCtxt<'tcx>,
    pub(crate) lint_store: &'tcx LintStore,
    /// All items should be created using the `alloc_*` functions. This ensures
    /// that we can later change the way we allocate and manage our memory
    buffer: &'ast bumpalo::Bump,
}

impl<'ast, 'tcx> std::fmt::Debug for RustcContext<'ast, 'tcx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RustcContext").finish()
    }
}

impl<'ast, 'tcx> DriverContext<'ast> for RustcContext<'ast, 'tcx> {
    fn warn(&self, s: &str, lint: &Lint) {
        self.tcx
    }

    fn warn_span(&self, s: &str, lint: &Lint, sp: &dyn Span<'_>) {
        todo!()
    }
}

impl<'ast, 'tcx> RustcContext<'ast, 'tcx> {
    pub fn alloc_with<F, T>(&self, f: F) -> &'ast T
    where
        F: FnOnce() -> T,
    {
        self.buffer.alloc_with(f)
    }

    #[must_use]
    pub fn alloc_slice_from_iter<T, I>(&self, iter: I) -> &'ast [T]
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        self.buffer.alloc_slice_fill_iter(iter)
    }

    #[must_use]
    pub fn alloc_slice<T, F>(&self, len: usize, f: F) -> &'ast [T]
    where
        F: FnMut(usize) -> T,
    {
        self.buffer.alloc_slice_fill_with(len, f)
    }
}

impl<'ast, 'tcx> RustcContext<'ast, 'tcx> {
    #[must_use]
    pub fn new_span(&'ast self, span: rustc_span::Span) -> &'ast dyn Span<'ast> {
        self.buffer.alloc_with(|| RustcSpan::new(span, self))
    }

    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn new_symbol(&'ast self, sym: rustc_span::symbol::Symbol) -> Symbol {
        Symbol::new(sym.as_u32())
    }

    #[must_use]
    pub fn new_ident(&'ast self, ident: rustc_span::symbol::Ident) -> &'ast Ident<'ast> {
        self.buffer
            .alloc_with(|| Ident::new(self.new_symbol(ident.name), self.new_span(ident.span)))
    }

    #[must_use]
    pub fn new_ty(&'ast self, kind: TyKind<'ast>, is_infered: bool) -> &'ast dyn Ty<'ast> {
        self.buffer.alloc_with(|| RustcTy::new(self, kind, is_infered))
    }

    pub fn new_lifetime(&'ast self) -> &'ast dyn Lifetime<'ast> {
        self.buffer.alloc_with(|| RustcLifetime {})
    }
}

impl<'ast, 'tcx> RustcContext<'ast, 'tcx> {
    #[must_use]
    pub fn new(ctx: LateContext<>, buffer: &'ast bumpalo::Bump) -> Self {
        Self {
            tcx,
            lint_store,
            buffer,
        }
    }
}
