use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use simplicity::dag::{Dag, DagLike, NoSharing};
use simplicity::jet::Core;
use simplicity::node::Inner;
use simplicity::types::Final;
use simplicity::{node, RedeemNode};

use crate::value::ExtValue;

pub type Expression = RedeemNode<Core>;

pub fn program_from_string(s: &str) -> Result<Arc<Expression>, String> {
    let empty_witness = HashMap::new();
    let forest = simplicity::human_encoding::Forest::parse(s).map_err(|e| e.to_string())?;
    forest
        .to_witness_node(&empty_witness)
        .ok_or("Main root is missing".to_string())?
        .finalize()
        .map_err(|e| e.to_string())
}

pub struct DisplayInner<'a, M: node::Marker>(&'a node::Node<M>);

impl<'a, M: node::Marker> From<&'a node::Node<M>> for DisplayInner<'a, M> {
    fn from(node: &'a node::Node<M>) -> Self {
        Self(node)
    }
}

impl<'a, M: node::Marker> fmt::Display for DisplayInner<'a, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.inner() {
            Inner::Iden => f.write_str("iden"),
            Inner::Unit => f.write_str("unit"),
            Inner::InjL(_) => f.write_str("injl"),
            Inner::InjR(_) => f.write_str("injr"),
            Inner::Take(_) => f.write_str("take"),
            Inner::Drop(_) => f.write_str("drop"),
            Inner::Comp(_, _) => f.write_str("comp"),
            Inner::Case(_, _) => f.write_str("case"),
            Inner::AssertL(_, _) => f.write_str("assertl"),
            Inner::AssertR(_, _) => f.write_str("assertr"),
            Inner::Pair(_, _) => f.write_str("pair"),
            Inner::Disconnect(_, _) => f.write_str("disconnect"),
            Inner::Witness(_) => f.write_str("witness"),
            Inner::Fail(_) => f.write_str("fail"),
            Inner::Jet(jet) => write!(f, "jet_{}", jet),
            Inner::Word(value) => write!(f, "const {}", ExtValue::from(value.as_ref())),
        }
    }
}

impl<'a, M: node::Marker> fmt::Debug for DisplayInner<'a, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub struct DisplayExpression<'a, M: node::Marker>(&'a node::Node<M>);

impl<'a, M: node::Marker> From<&'a node::Node<M>> for DisplayExpression<'a, M> {
    fn from(node: &'a node::Node<M>) -> Self {
        Self(node)
    }
}

impl<'a, M: node::Marker> fmt::Display for DisplayExpression<'a, M>
where
    &'a node::Node<M>: DagLike,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for data in self.0.verbose_pre_order_iter::<NoSharing>() {
            match data.n_children_yielded {
                1 => {
                    if let Dag::Binary(..) = data.node.inner().as_dag() {
                        f.write_str(" ")?;
                    } else {
                        debug_assert!(matches!(data.node.inner().as_dag(), Dag::Unary(..)));
                    }
                }
                2 => {
                    debug_assert!(matches!(data.node.inner().as_dag(), Dag::Binary(..)));
                    f.write_str(")")?;
                }
                n => {
                    debug_assert!(n == 0, "Combinators are nullary, unary or binary");
                    write!(f, "{}", DisplayInner::from(data.node))?;
                    match data.node.inner().as_dag() {
                        Dag::Unary(..) => f.write_str(" ")?,
                        Dag::Binary(..) => f.write_str(" (")?,
                        _ => {}
                    }
                }
            };
        }

        Ok(())
    }
}

impl<'a, M: node::Marker> fmt::Debug for DisplayExpression<'a, M>
where
    &'a node::Node<M>: DagLike,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub fn pad_left(a: &Final, b: &Final) -> usize {
    std::cmp::max(a.bit_width(), b.bit_width()) - a.bit_width()
}

pub fn pad_right(a: &Final, b: &Final) -> usize {
    std::cmp::max(a.bit_width(), b.bit_width()) - b.bit_width()
}
