# Vesper: declarative human-readable structural language, better than XML

![Build](https://github.com/UBIDECO/vesper/workflows/Build/badge.svg)
![Tests](https://github.com/UBIDECO/vesper/workflows/Tests/badge.svg)
![Lints](https://github.com/UBIDECO/vesper/workflows/Lints/badge.svg)
[![codecov](https://codecov.io/gh/UBIDECO/vesper/branch/master/graph/badge.svg)](https://codecov.io/gh/UBIDECO/vesper)

[![crates.io](https://img.shields.io/crates/v/vesper-lang)](https://crates.io/crates/vesper-lang)
[![Docs](https://docs.rs/vesper-lang/badge.svg)](https://docs.rs/vesper-lang)
[![Apache-2 licensed](https://img.shields.io/crates/l/vesper-lang)](./LICENSE)

## Overview

Sometimes we need to represent multidimensional data hierarchies in a very laconic style,
which will be easily-readable for humans.
At the same time, we do not want to lose precision 
and have this representation formal and deterministic.

Up to day, the choice had to be made between JSON, YAML and XML.
JSON lacks multidimensionality and, while being flexible,
lacks a proper efficient way to define data formal structure (schema).
Yes, JSON schemata do exist, but are very clunky — like XML.
XML has more tools to formalize the data structure, but is quite hard to visually parse.
YAML is the most parsable, it also supports more structure than JSON —
but still less than XML, and again, lacks efficient schema languages.

Thus, we created Vesper: a formal language which is visually clean 
and at the same time has the same power as XML:

```vesper
rec Transaction
    enum Version: U8 v1 1, v2 2
    list Inputs: len 0..MAX64
        rec PrevOut
            bytes Txid: len 32
            field Vout: U32
        field Sequence: U32
        bytes ScriptSig: len 0..MAX64
        list Witness: len 0..MAX64
            bytes ByteStr: len 0..MAX64
    list Outputs: len 0..MAX64
        field Value: U64
        bytes ScriptPubkey: len 0..MAX64
    field LockTime: U32
```

The above is the full representation of both data type hierarchy,
semantic structure and memory layout for a Bitcoin transaction:
this is how clean it can be.
Written in JSON, it would take at least twice as much text,
plenty of quotation marks, brackets and braces —
and, the most important, it would not be possible to express
two-dimensional information of type attributes and structure fields.

XML equivalent is also much more verbose and less readable:

```xml
<rec name="Transaction">
    <enum name="Version" type="U8">
        <variant tag="V1" value="1"/>
        <variant tag="V2" value="2"/>
    </enum>
    <list name="Inputs" min-len="0" max-len="MAX64">
        <rec name="PrevOut">
            <bytes name="Txid" len="32"/>
            <field name="Vout" type="U32"/>
        </rec>
        <field name="Sequence" type="U32"/>
        <bytes name="ScriptSig" min-len="0" max-len="MAX64"/>
        <list name="Witness" min-len="0" max-len="MAX64">
            <bytes name="ByteStr" min-len="0" max-len="MAX64"/>
        </list>
    </list>
    <list name="Outputs" min-len="0" max-len="MAX64">
        <field name="Value" type="U64"/>
        <bytes name="ScriptPubkey" min-len="0" max-len="MAX64"/>
    </list>
    <field name="LockTime" type="U32"/>
</rec>
```

Vesper uses new clause notation, called *T-expressions*.
You may be aware of [S-expressions] and [M-expressions],
and *T-expressions* are the new guest in the town,
which follows a specific visually clean pattern
to represent semantic constructs:

```
<predicate> <subject>: <attributes>,*
    <content>
```

where predicate-subject-attributes go in one line, forming a kind of sentence,
and content is nested below and can span multiple lines.
Or, for a short content, one may use a one-liner:

```
<predicate> <subject>: <attributes>,* := <content>
```

Each line of the content is, in fact, another V-expression,
so we end up with a tree
(here is why the expression is called "T", i.e. "Tree-expression").

The full grammar of Vesper is so simple,
that its formal definition can fit just nine lines
(here we use our custom BNF-styled notation):
```
t-expr => t-expr-multiline | t-expr-one-line

t-expr-multiline => predicate subject `:` ( attr ),* 
            ( t-expr )\*

t-expr-one-line => predicate subject `:` ( attr ),* `:=` t-expr

subject => ident
predicate => ident
attr => simple | named
simple => ident | expr
named => ident ( ident | expr )

ident => (\w_)[\w\d_-]*
expr => [\S]+ -- all ASCII printable symbols except of whitespace
```

Vesper comes with its own schema language (named "Vesper schema"),
which allows creating domain-specific sublanguages,
like we did for the data type and memory layout above:

```vesper-schema
rec := - (*)
tuple := - (*)
as := \type (-)
enum := \type? \ident+=\int (-)
union := \type? (*)
bytes := \range (-)
list := \range (+)
char := - (-)
str := \range (-)
```

As you see, it is also ultra-concise:
the whole definition for an arbitrary data type hierarchy and memory layout
fits in just nine lines!


## Contributing

[CONTRIBUTING.md](../CONTRIBUTING.md)

## License

The libraries are distributed on the terms of [Apache 2.0 license](LICENSE).

[S-expressions]: https://en.wikipedia.org/wiki/S-expression
[M-expressions]: https://en.wikipedia.org/wiki/M-expression

