use crate::syntax::*;

use crate::config::Config;
use crate::r#type::FeType;
use crate::type_resolver::Scope;

use crate::result::Result;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct FeBorrowChecker {
    cfg: Arc<Config>,
    pkg: Arc<Mutex<FeSyntaxPackage<FeType>>>,
}

impl FeBorrowChecker {
    pub fn check_package(cfg: Arc<Config>, pkg: Arc<Mutex<FeSyntaxPackage<FeType>>>) -> Result {
        return Self::new(cfg, pkg).check();
    }

    pub fn new(cfg: Arc<Config>, pkg: Arc<Mutex<FeSyntaxPackage<FeType>>>) -> Self {
        return Self { cfg, pkg };
    }

    pub fn check(mut self) -> Result {
        /*
            Thoughts:
             - impl Stmt & Decl visitor on FeFileBorrowChecker
             - maintain running scope
             - impl Expr visitor on FeExprBorrowChecker
             - if ident is owned, mark as moving
                - parent call resolves the moving to either moved or borrowed
                - ie. if fn call where the arg only needs &, then moving is resolved to borrow
                - ie. if &mut ident, then moving is resolved to mutable borrow
                - ie. if assign to val without explicit type, then moving is resolved to moved
             - moved idents cannot later be referenced (maybe some exceptions)

        const name = "owned"
        const name1 = $name  // <-- move
        const name2 = $name1 // <-- borrow and unborrow
        */

        dbg!(&self);
        todo!();
    }
}

struct FeFileBorrowChecker {
    scope: Scope,
}

enum MoveState {
    NoneYet,

    ImmutRef,
    MutRef,

    /*
    is_new is if we know whether or not the share wrapper
    is being created on non-shared data, or is just being shared / cloned
    ie.
        const value = $"foo"   // new
        const value: $ = "foo" // new

        const other = $value   // not new
    */
    ImmutShare { is_new: Option<bool> },
    MutShare { is_new: Option<bool> },

    /*
    confident is if we are or are not confident that the data is being moved
    ie.
        fn read(_: &'T) -> noop
        fn consume(_: 'T) -> noop

        const value = "foo"

        // looks like a move, but not confident
        // ... after matching the arg with the & param: Move -> ImmutRef
        read(value)

        // looks like a move, but not confident
        // ... after matching the arg with the param, confident = true
        // once confident = true, cannot transition away
        consume(value)
    */
    Move { confident: bool },
}

struct FeExprBorrowChecker {
    interactions: HashMap<Arc<str>, MoveState>,
}

impl ExprVisitor<FeType> for FeExprBorrowChecker {
    fn visit_bool_literal_expr(&mut self, expr: Arc<Mutex<BoolLiteralExpr<FeType>>>) {}

    fn visit_number_literal_expr(&mut self, expr: Arc<Mutex<NumberLiteralExpr<FeType>>>) {}

    fn visit_plain_string_literal_expr(
        &mut self,
        expr: Arc<Mutex<PlainStringLiteralExpr<FeType>>>,
    ) {
    }

    fn visit_fmt_string_literal_expr(&mut self, expr: Arc<Mutex<FmtStringLiteralExpr<FeType>>>) {
        let expr = &*expr.try_lock().unwrap();

        for part in &expr.rest {
            let part_expr = &*part.expr.0.try_lock().unwrap();

            part_expr.accept(self);

            todo!("maybe apply non-move somehow if it's just stringification?");
        }
    }

    fn visit_ident_expr(&mut self, expr: Arc<Mutex<IdentExpr<FeType>>>) {
        todo!("set initial interaction depending on type")
    }

    fn visit_call_expr(&mut self, expr: Arc<Mutex<CallExpr<FeType>>>) {
        let expr = &*expr.try_lock().unwrap();

        todo!("check callee & args, be weary of implicit transformations")
    }

    fn visit_unary_expr(&mut self, expr: Arc<Mutex<UnaryExpr<FeType>>>) {
        todo!("apply refs and shares, and maybe ops like 'not' should remove any potential 'moves'")
    }

    fn visit_binary_expr(&mut self, expr: Arc<Mutex<BinaryExpr<FeType>>>) {
        todo!("apply ops")
    }

    fn visit_static_ref_expr(&mut self, expr: Arc<Mutex<StaticRefExpr<FeType>>>) {
        todo!()
    }

    fn visit_construct_expr(&mut self, expr: Arc<Mutex<ConstructExpr<FeType>>>) {
        todo!()
    }

    fn visit_get_expr(&mut self, expr: Arc<Mutex<GetExpr<FeType>>>) {
        todo!()
    }

    fn visit_if_expr(&mut self, expr: Arc<Mutex<IfExpr<FeType>>>) {
        todo!()
    }

    fn visit_loop_expr(&mut self, expr: Arc<Mutex<LoopExpr<FeType>>>) {
        todo!()
    }

    fn visit_while_expr(&mut self, expr: Arc<Mutex<WhileExpr<FeType>>>) {
        todo!()
    }
}
