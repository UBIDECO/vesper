// Vesper: declarative human-readable structural language
//
// SPDX-License-Identifier: Apache-2.0
//
// Designed & Written in 2024-2025 by
//     Dr Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright (C) 2024-2025 Laboratories for Ubiquitous and Deterministic Computing,
//     Institute for Distributed and Cognitive Systems, Lugano, Switzerland
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::{Display, Formatter};

use amplify::confinement::{SmallVec, TinyVec};
use strict_encoding::Ident;

pub trait Predicate: Clone + Eq {
    type Attr: Attribute;
}

pub trait Expression: Clone + Eq + Display {}

pub trait Attribute: Clone + Eq {
    type Expression: Expression;

    fn is_named(&self) -> bool { self.name().is_some() }
    fn name(&self) -> Option<Ident>;
    fn value(&self) -> AttrVal<Self::Expression>;
}

#[derive(Clone, Eq, PartialEq, Debug, Display)]
#[display(inner)]
pub enum AttrVal<E: Expression> {
    Ident(Ident),
    Expr(E),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TExpr<P: Predicate> {
    pub subject: Ident,
    pub predicate: P,
    pub attributes: SmallVec<P::Attr>,
    pub content: TinyVec<Box<TExpr<P>>>,
    pub comment: Option<String>,
}

impl<P: Predicate> TExpr<P> {
    pub fn display(&self) -> TExprDisplay<'_, P>
    where P: Display {
        TExprDisplay {
            expr: self,
            indent: 0,
            tab: s!("  "),
        }
    }
}

pub struct TExprDisplay<'expr, P: Predicate>
where P: Display
{
    expr: &'expr TExpr<P>,
    indent: usize,
    tab: String,
}

impl<'expr, P: Predicate> TExprDisplay<'expr, P>
where P: Display
{
    pub fn indented(parent: &Self, expr: &'expr TExpr<P>) -> TExprDisplay<'expr, P> {
        TExprDisplay {
            expr,
            indent: parent.indent + 1,
            tab: parent.tab.clone(),
        }
    }
}

impl<'expr, P: Predicate> Display for TExprDisplay<'expr, P>
where P: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const MAX_LINE_VARS: usize = 8;

        let expr = self.expr;
        let attrs = &expr.attributes;

        let indent = self.tab.repeat(self.indent);
        write!(f, "{indent}{} {}", expr.predicate, expr.subject)?;

        if !attrs.is_empty() {
            f.write_str(": ")?;
        }

        let mut iter = expr.attributes.iter().enumerate().peekable();
        while let Some((pos, attr)) = iter.next() {
            if let Some(name) = attr.name() {
                write!(f, "{name} ")?;
            }
            write!(f, "{}", attr.value())?;

            if iter.peek().is_some() {
                f.write_str(",")?;
            }
            if pos > 0 && pos % MAX_LINE_VARS == 0 {
                write!(f, "\n{indent}{}", self.tab)?;
            } else {
                f.write_str(" ")?;
            }
        }

        if let Some(comment) = &expr.comment {
            write!(f, " -- {comment}")?;
        }
        writeln!(f)?;

        for expr in &expr.content {
            let display = TExprDisplay::indented(self, expr.as_ref());
            Display::fmt(&display, f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Display)]
    #[display("predicate")]
    struct Test;
    impl Predicate for Test {
        type Attr = TestAttr;
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Display)]
    enum TestAttr {
        #[display("attr1")]
        TestAttr1,
        #[display("attr2")]
        TestAttr2,
    }
    impl Attribute for TestAttr {
        type Expression = String;

        fn name(&self) -> Option<Ident> {
            Some(Ident::from_str(&self.to_string()).unwrap())
        }

        fn value(&self) -> AttrVal<Self::Expression> {
            AttrVal::Expr("value".to_string())
        }
    }

    impl Expression for String {}

    #[test]
    fn display() {
        let expr = TExpr {
            subject: strict_encoding::ident!("test"),
            predicate: Test,
            attributes: small_vec![ TestAttr::TestAttr1, TestAttr::TestAttr2 ],
            content: Default::default(),
            comment: None,
        };
        assert_eq!(expr.display().to_string(), "predicate test: attr1 value, attr2 value \n");
    }
}
