<br />
<div align="center">
	<div>
		<img alt="Ruleset: Monotonic Chart Parsing" src="assets/logo.svg" width="300px" height="100px" />
	</div>
	<div>
		<a ref="https://crates.io/crates/ruleset"><img alt="crates.io" src="https://img.shields.io/crates/v/ruleset"></a>
		<a ref="https://docs.rs/ruleset/latest/ruleset/"><img alt="docs.rs" src="https://img.shields.io/docsrs/ruleset"></a>
		<a ref="https://github.com/yuhr/ruleset/blob/develop/LICENSE"><img alt="License" src="https://img.shields.io/github/license/yuhr/ruleset"></a>
	</div>
</div>
<br /><br />

Ruleset is a **[bottom-up](https://en.wikipedia.org/wiki/Bottom-up_parsing)** **[S-attributed](https://en.wikipedia.org/wiki/S-attributed_grammar)** **[online](https://en.wikipedia.org/wiki/Online_algorithm)** **[forward-chaining](https://en.wikipedia.org/wiki/Forward_chaining)** **[CSG](https://en.wikipedia.org/wiki/Context-sensitive_grammar)** parser library for Rust, meant to be one of the most general parsing frameworks in the world.

Our algorithm constructs well-formed substring tables (so-called “[charts](https://en.wikipedia.org/wiki/Chart_parser)”) quite similar to the [CYK algorithm](https://en.wikipedia.org/wiki/CYK_algorithm), but with the essence of [Earley parsing](https://en.wikipedia.org/wiki/Earley_parser). Compared to well-known parsing techniques, we achieved multiple improvements, IBNLT:

- Normalizations such as [CNF](https://en.wikipedia.org/wiki/Chomsky_normal_form) and [KNF](https://en.wikipedia.org/wiki/Kuroda_normal_form) are no longer required
- [Factorings](https://en.wikipedia.org/wiki/LL_parser#Left_factoring) and [eliminations](https://en.wikipedia.org/wiki/Left_recursion#Removing_left_recursion) are no longer required
- [Monotonic grammars](https://en.wikipedia.org/wiki/Noncontracting_grammar), thus context-sensitive grammars, are supported
- Synthesized attributes i.e. computations & conditionals are supported
- Online parsing i.e. streamed inputs, undos and redos are supported

Targeted applications:

- Experimenting with formal grammars
- Implementing parsers for natural or constructed languages

Note that Ruleset is _not_ a parser code generator, but rather a parser runtime with a set of APIs to build and manipulate parsers dynamically.

One of the drawbacks of Ruleset is that it uses proc-macro to define grammars; due to the [current limitation](https://github.com/rust-lang/rustfmt/issues/8) of `rustfmt`, your grammar won't be auto-formatted. Another drawback is that the macro expands into Rust's pattern-matching code, so there's no obvious way to perform goal-directed operations like prediction.

Currently, this library is only a PoC implementation in the [millionaires' programming](https://scrapbox.io/masui/%E5%AF%8C%E8%B1%AA%E7%9A%84%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0) style; any kind of substantial performance optimizations are yet to be done.

## Installation

```sh
cargo add ruleset
```

## Why

**Capability**. By supporting [monotonic grammars](https://en.wikipedia.org/wiki/Noncontracting_grammar), enumerating every possible parse trees, and operating online, Ruleset aims to be a “safety net” where your work-in-progress grammar might grow context-sensitive, ambiguous, modified dynamically, or **just in case you're too lazy to worry about it**.

**Visualizability**. Ruleset maintains the internal parsing state as an easy-to-visualize two-dimensional table, so at any time in the midst of parsing, you can inspect the internal state to understand how your grammar is working. This largely helps when you were to design, formalize, or debug your grammar.

**Simplicity**. The syntax of the `ruleset!` macro which is to write grammars is kept as straightforward as possible in order to reduce requirement for you to self-educate how to read/write the syntax specific to this library. There're only a few notations that have irregular usage of Rust symbols.

In the wild there must definitely be more efficient parsing algorithms and implementations optimized for more specific, restricted, or static grammars, but Ruleset is not going to sacrifice the above three goals for the sake of performance, thereby being rather suitable for experimentations or reference implementations than performance-intensive production applications.

## Usage

In this document, we use the term “syntactic rules” to refer to rules defined in `ruleset!` macro, which have the inverted form compared to how rules are typically written in formal grammar studies. For example, `A, B => C, D` in Ruleset corresponds to $\mathrm{C} \mathrm{D} → \mathrm{A} \mathrm{B}$ in formal grammar.

### Basics

```rust
use ruleset::prelude::*;

// 0. Instantiate an empty parser
let mut parser = Parser::new();

// 1. Create syntactic rules
let ruleset = ruleset! {
	n, m => n + m;
};

// 2. Process inputs, enabling the grammar
parser.parse_all([0, 1, 2], &[&ruleset]);

// 3. Search for the desired syntax trees
let trees = parser.interpret_as(3).collect_trees();
assert!(trees.len() == 2);

// 4. Inspect the contents of the trees
for tree in trees {
	println!("{tree}");
}
```

It will print this:

```plaintext
3@0..3
├─0@0..1
├─1@1..2
└─2@2..3
3@0..3
├─1@0..2
│ └─0@0..1
└─2@2..3
```

Looks wrong? That's because `ruleset::Tree` automatically flattens recursive occurence of nodes with the same symbol (under the `Hash` equality). This behavior is what users would intend in general, as formal grammars almost always include variable-length rules e.g. combination of `B => A` and `A, B => A` (Kleene plus). Without flattening, the trees would look like:

```plaintext
3@0..3
├─0@0..1
└─3@1..3
	├─1@1..2
	└─2@2..3
3@0..3
├─1@0..2
│ ├─0@0..1
│ └─1@1..2
└─2@2..3
```

So, if you want the grammar to be more explicit about levels of association:

```rust
use ruleset::prelude::*;
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Symbol { Sum(usize, usize), Usize(usize) }
use Symbol::*;
impl From<usize> for Symbol { fn from(value: usize) -> Self { Usize(value) } }
impl std::fmt::Display for Symbol { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Debug::fmt(&self, f) } }

let mut parser = Parser::new();
let ruleset = ruleset! {
	Sum(l, n), Usize(m) => Sum(l + 1, n + m);
	Usize(n), Usize(m) => Sum(0, n + m);
};
parser.parse_all([0, 1, 2], &[&ruleset]);
let trees = parser.interpret().filter(by_matches!(Sum(_, 3))).collect_trees();
assert!(trees.len() == 1);
for tree in trees {
	println!("{tree}");
}
```

And you'll get this:

```plaintext
Sum(1, 3)@0..3
├─Sum(0, 1)@0..2
│ ├─Usize(0)@0..1
│ └─Usize(1)@1..2
└─Usize(2)@2..3
```

You can also see the [tests](tests) for further usages.

### Reading Chart

When you run `println!("{parser}")` after the above example, a table similar to the following one should be printed:

```plaintext
┌─────────────────────────────┬─────────────────────────────────┬─────────────────────────────────┐
│ ϛF399                       ┆                                 ┆                                 │
│ └─Usize(0)                  ┆                                 ┆                                 │
│ Δ05F3∵2387⋅F399             ┆                                 ┆                                 │
│ └─Usize(m) => Sum(0, n + m) ┆                                 ┆                                 │
│                             ┆                                 ┆                                 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ ϛ0462                       ┆ Δ2BAA∵05F3:0462                 ┆                                 │
│ └─Usize(1)                  ┆ └─=> Sum(0, n + m)              ┆                                 │
│ Δ7DF4∵2387⋅0462             ┆ ϛ22F6∵2BAA                      ┆                                 │
│ └─Usize(m) => Sum(0, n + m) ┆ └─Sum(0, 1)                     ┆                                 │
│                             ┆ Δ3C49∵8DF6⋅22F6                 ┆                                 │
│                             ┆ └─Usize(m) => Sum(l + 1, n + m) ┆                                 │
│                             ┆                                 ┆                                 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ ϛE929                       ┆ Δ8B43∵7DF4:E929                 ┆ ΔD9DA∵3C49:E929                 │
│ └─Usize(2)                  ┆ └─=> Sum(0, n + m)              ┆ └─=> Sum(l + 1, n + m)          │
│ ΔC910∵2387⋅E929             ┆ ϛD5EB∵8B43                      ┆ ϛ2363∵D9DA                      │
│ └─Usize(m) => Sum(0, n + m) ┆ └─Sum(0, 3)                     ┆ └─Sum(1, 3)                     │
│                             ┆ ΔECCC∵8DF6⋅D5EB                 ┆ Δ09CD∵8DF6⋅2363                 │
│                             ┆ └─Usize(m) => Sum(l + 1, n + m) ┆ └─Usize(m) => Sum(l + 1, n + m) │
│                             ┆                                 ┆                                 │
└─────────────────────────────┴─────────────────────────────────┴─────────────────────────────────┘
```

Two lines of text connected by `└─` is an **item**. An item is either a **reducer** `ϛ` or a **reduction** `Δ`. The 4-digit code after the Greek character is an opaque ID of the item. The second line represents the content of the item.

A reducer is either an **input** or a **conclusion** (a consequent symbol originated from the right hand side of a syntactic rule). When a reducer is plotted into the chart, it shall cause reductions as per the given grammar and the current context of the chart (these two corresponds to the notion of “[state set](https://en.wikipedia.org/wiki/Earley_parser#The_algorithm)” in the Earley's algorithm).

A reduction represents a single step of derivation, meaning an internal node of a parse tree. By nature, it contains two parents being a **reducible** (a.k.a. a subrule; a non-zero-antecedent suffix of a syntactic rule) and a reducer, and contains one child being a **reductum**, which is either another reducible or a consequence (a zero-antecedent suffix of a syntactic rule). This is why the ID of a reduction is followed by the “because” symbol `∵` and two IDs, each representing the reducible and the reducer respectively. The separator `⋅` or `:` indicates whether the reducible was a rule (i.e. derived direcly from the grammar) or a strict subrule (i.e. derived from the context of the chart) respectively.

Reductions with the reductum being a consequence are called conclusive. For instance in the above table `Δ2BAA`, `Δ8B43`, and `ΔD9DA` are conclusive. Technically, conclusive reductions don't directly cause further reductions. They instead plot the conclusions to the chart at the appropriate cell for each. This is why the ID of a reducer is sometimes followed by the `∵` and one ID, representing the conclusive reduction.

As you may anticipate, textual representations of tables immediately explode when given many inputs. We would like to provide a way to output internal data in an interoperable format, and an HTML frontend to inspect them, in the near future.