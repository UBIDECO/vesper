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

use std::fmt::{Display, Formatter};

use amplify::confinement::{SmallVec, TinyVec};
use strict_encoding::Ident;

pub trait Vocabulary {
    type Subj;
    type Pred;
    type Attr;
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Display)]
pub enum AttrType<A: Display> {
    #[display(inner)]
    Unnamed(A),

    #[display("{0}={1}")]
    Named(Ident, A),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TExpr<V: Vocabulary> {
    pub subject: V::Subj,
    pub predicate: V::Pred,
    pub attributes: SmallVec<V::Attr>,
    pub content: TinyVec<Box<TExpr<V>>>,
}

impl<V: Vocabulary> TExpr<V> {
    pub fn display(&self) -> TExprDisplay<V>
    where
        V::Subj: Display,
        V::Pred: Display,
        V::Attr: Display,
    {
        TExprDisplay {
            expr: self,
            indent: 0,
            tab: s!("  "),
        }
    }
}

pub struct TExprDisplay<'expr, V: Vocabulary>
where
    V::Subj: Display,
    V::Pred: Display,
    V::Attr: Display,
{
    expr: &'expr TExpr<V>,
    indent: usize,
    tab: String,
}

impl<'expr, V: Vocabulary> TExprDisplay<'expr, V>
where
    V::Subj: Display,
    V::Pred: Display,
    V::Attr: Display,
{
    pub fn indented(parent: &Self, expr: &'expr TExpr<V>) -> TExprDisplay<'expr, V> {
        TExprDisplay {
            expr,
            indent: parent.indent + 1,
            tab: parent.tab.clone(),
        }
    }
}

impl<'expr, V: Vocabulary> Display for TExprDisplay<'expr, V>
where
    V::Subj: Display,
    V::Pred: Display,
    V::Attr: Display,
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
