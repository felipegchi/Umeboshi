use std::rc::Rc;

use loaf_core::types::Level;
use loaf_core::value::{Closure, Stuck, Value};

use crate::context::{force, Context};
use crate::eval::apply;
use Value::*;

fn apply_to(closure: &Closure, name: Option<String>, lvl: Level) -> Rc<Value> {
    apply(closure, name, Rc::new(Value::var(lvl)))
}

fn conv_spine(ctx: &Context, lvl: Level, left: &[Rc<Value>], right: &[Rc<Value>]) -> bool {
    if left.len() != right.len() {
        false
    } else {
        for i in 0..left.len() {
            if !conv(ctx, lvl, left[i].clone(), right[i].clone()) {
                return false;
            }
        }
        true
    }
}

pub fn conv_stuck(stuck: &Stuck, stuck1: &Stuck) -> bool {
    match (stuck, stuck1) {
        (Stuck::Rigid(x), Stuck::Rigid(y)) => x.0 == y.0,
        (Stuck::Data(_, x), Stuck::Data(_, y)) => x.0 == y.0,
        (Stuck::Fun(_, x), Stuck::Fun(_, y)) => x.0 == y.0,
        (Stuck::Const(_, x), Stuck::Const(_, y)) => x.0 == y.0,
        _ => false
    }
}

pub fn conv(ctx: &Context, lvl: Level, left: Rc<Value>, right: Rc<Value>) -> bool {
    match (&*force(ctx, left), &*force(ctx, right)) {
        (Universe, Universe) => true,
        (Neutral(s, sp), Neutral(f, sp1)) if conv_stuck(s, f) => conv_spine(ctx, lvl, sp, sp1),
        (Pi(n, t, b), Pi(n1, t1, b1)) => conv(ctx, lvl, t.clone(), t1.clone()) && conv(ctx, lvl.inc(), apply_to(b, n.clone(), lvl), apply_to(b1, n1.clone(), lvl)),
        (Lam(n, b), Lam(n1, b1)) => conv(ctx, lvl.inc(), apply_to(b, Some(n.clone()), lvl), apply_to(b1, Some(n1.clone()), lvl)),
        (Sigma(n, t, b), Sigma(n1, t1, b1)) => conv(ctx, lvl, t.clone(), t1.clone()) && conv(ctx, lvl.inc(), apply_to(b, n.clone(), lvl), apply_to(b1, n1.clone(), lvl)),
        (Pair(f, s), Pair(f1, s1)) => conv(ctx, lvl, f.clone(), f1.clone()) && conv(ctx, lvl, s.clone(), s1.clone()),
        (Left(t), Left(t1)) => conv(ctx, lvl, t.clone(), t1.clone()),
        (Right(t), Right(t1)) => conv(ctx, lvl, t.clone(), t1.clone()),
        (_, _) => false,
    }
}
