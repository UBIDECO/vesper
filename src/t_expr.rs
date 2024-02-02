// Vesper: declarative human-readable structural language
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2024 by
//     Dr Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright (C) 2024 UBIDECO Institute, Switzerland
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

use std::fmt;
use std::fmt::{Display, Formatter};

use amplify::confinement::{SmallVec, TinyVec};
use strict_encoding::Ident;

pub trait Predicate: Clone + Eq {
    type Attr: Attribute;
    type AttrNamed: AttributeNamed;
}
pub trait Attribute: Clone + Eq {
    fn value(&self) -> Ident;
}

pub trait AttributeNamed: Attribute {
    fn name(&self) -> Ident;
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Attr<U: Attribute, N: AttributeNamed> {
    Unnamed(U),
    Named(N),
}

impl<U: Attribute, N: AttributeNamed> Display for Attr<U, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Attr::Unnamed(a) => write!(f, "{}", a.value()),
            Attr::Named(a) => write!(f, "{}~{}", a.name(), a.value()),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct TExpr<P: Predicate> {
    pub subject: Ident,
    pub predicate: P,
    pub attributes: SmallVec<Attr<P::Attr, P::AttrNamed>>,
    pub content: TinyVec<Box<TExpr<P>>>,
}

impl<P: Predicate> TExpr<P> {
    pub fn display(&self) -> TExprDisplay<P>
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
        let expr = self.expr;

        for _ in 0..self.indent {
            f.write_str(&self.tab)?;
        }
        write!(f, "{} {}", expr.subject, expr.predicate)?;
        for attr in &expr.attributes {
            write!(f, " {attr}")?;
        }
        writeln!(f)?;
        for expr in &expr.content {
            let display = TExprDisplay::indented(self, expr.as_ref());
            Display::fmt(&display, f)?;
        }
        Ok(())
    }
}
