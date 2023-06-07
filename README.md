# ari

<img src="ari.svg" width="128px" height="128px">

A type-centred [purely functional
programming](https://en.wikipedia.org/wiki/Purely_functional_programming)
language designed to type binary files.

Funding for Ari is provided by [NLnet](https://nlnet.nl) and the
European Commission through the [NGI
Assure](https://www.assure.ngi.eu) initiative.

## Why does this exist?

### To make binary files more accessible

Most binary files require specially designed tools to read & write,
but ari types are designed so that you can decompose any binary file
down into its component parts. The language is intended to be a
foundation for other tools to build on top of.

Some of the tools we plan on building with ari include:

#### arie

An editor specially designed to edit files using ari types.

We'll also build plugins for existing text editors.

#### ariq

A command line tool to query components of a file using ari types.

#### aric

A command line tool to compile ari types into other languages using
[ari map types](#exponentiate-and-map-expressions).

#### arid

A command line tool to structurally diff files of the same ari type.

#### ariz

A command line tool to compress files & directories using ari types.

Ari's type system is designed to act as a guide for [arithmetic
coding](https://en.wikipedia.org/wiki/Arithmetic_coding).

### ... but also, not just binary formats

While the primary focus is to help interpret binary data, ari is also
designed to model grammars for text-based languages.

Here is a quick comparison of how some formal language concepts map
into ari:

> **NOTE:** This comparison is meant for people who are already
> familiar with these concepts.
>
> If you want to get a better understanding of what they mean, or what
> the ari equivalents mean, see the [language
> reference](#language-reference).

<table>
<tr>

<td>
<table>
<tr><th>Regex</th><th>Ari</th></tr>

<tr>
<td>

```regex
a|b|c
```

</td>
<td>

```lisp
(| "a" "b" "c")
```

</td>
</tr>

<tr>
<td>

```regex
abc
```

</td>
<td>

```lisp
"abc"
```

or

```lisp
(* "a" "b" "c")
```

</td>
</tr>

<tr>
<td>

```regex
a?
```

</td>
<td>

```lisp
(^ "a" ?)
```

</td>
</tr>

<tr>
<td>

```regex
a*
```

</td>
<td>

```lisp
(^ "a" _)
```

</td>
</tr>

<tr>
<td>

```regex
a+
```

</td>
<td>

```lisp
(^ "a" !)
```

</td>
</tr>

<tr>
<td>

```regex
a{3}
```

</td>
<td>

```lisp
(^ "a" 3)
```

</td>
</tr>

<tr>
<td>

```regex
a{2, 5}
```

</td>
<td>

```lisp
(^ "a" (.. 2 5))
```

or

```lisp
(^ "a" (| 2 3 4))
```

</td>
</tr>

<tr>
<td>

```regex
.
```

</td>
<td>

```lisp
codepoint
```

</td>
</tr>

</table>
</td>

<td>
<table>
<tr><th>Backus–Naur form</th><th>Ari</th></tr>

<tr>
<td>

```bnf
"terminal"
```

</td>
<td>

```lisp
"terminal"
```

</td>
</tr>

<tr>
<td>

```bnf
<non-terminal>
```

</td>
<td>

```lisp
non-terminal
```

</td>
</tr>

<tr>
<td>

```bnf
<symbol> ::= "ab" | "cd" <x>
```

</td>
<td>

```lisp
:symbol (| "ab" (* "cd" x))
```

</td>
</tr>

</table>
</td>

</tr>
</table>

## What makes ari unique?

### Types are the main focus of the language

In ari, everything is a type. Types and type expressions are the
primary focus of the language. You can add, multiply, and even
exponentiate types, much like you can a number in any other
programming language.

<table>
<tr><th>Ari type expressions</th><th>Equivalent types</th></tr>

<tr>
<td>

```lisp
:sum (+ a b c)
```

</td>
<td>

- [enum](https://en.wikipedia.org/wiki/Enumerated_type)
- [tagged union](https://en.wikipedia.org/wiki/Tagged_union)

</td>
</tr>

<tr>
<td>

```lisp
:product (* a b c)
```

</td>
<td>

- [record, struct,
  tuple](<https://en.wikipedia.org/wiki/Record_(computer_science)>)
- [class](<https://en.wikipedia.org/wiki/Class_(computer_programming)>)
- [trait](<https://en.wikipedia.org/wiki/Trait_(computer_programming)>)
- [concept,
  interface](<https://en.wikipedia.org/wiki/Concept_(generic_programming)>)
- [protocol](<https://en.wikipedia.org/wiki/Protocol_(object-oriented_programming)>)

</td>
</tr>

<tr>
<td>

```lisp
:map (^ a b c)
```

</td>
<td>

- [function type
  signature](https://en.wikipedia.org/wiki/Type_signature)
- [associative array, map,
  dictionary](https://en.wikipedia.org/wiki/Associative_array)
- [array type](https://en.wikipedia.org/wiki/Array_data_type)
- [stack](https://en.wikipedia.org/wiki/Stack_%28abstract_data_type%29)
- [matrix](<https://en.wikipedia.org/wiki/Matrix_(mathematics)>)
- [vector space](https://en.wikipedia.org/wiki/Vector_space)
- [tensor](https://en.wikipedia.org/wiki/Tensor)

</td>
</tr>
</table>

See the [language reference](#language-reference)

### Generalizes the idea of primitive types

In ari, there are infinitely many primitives types generalized by a
single concept, [natural
numbers](https://en.wikipedia.org/wiki/Natural_number). We call these
primitive types [natural types](#natural-expressions).

...well what exactly does this mean? Say you want to create a `size`
type that has 3 possible states, `small`, `medium`, `large`. In ari,
you can precisely type the number of states using the `3` type:

```lisp
:size 3
```

The possible values of this type are `0`, `1`, `2`, which you can use
to represent `small`, `medium`, `large`. This is a powerful concept,
but this small example doesn't fit with the main goal of ari, to make
things more accessible.

...this is exactly where [algebraic
expressions](#algebraic-expressions) and [labels](#label-expressions)
come in! You can break down and label the individual states of a `3`
type with a [sum expression](#additive-expressions). For example, this
sum expression produces a sum type that's equivalent to the `3` type:

```lisp
:size (+ :small 1 :medium 1 :large 1)
```

and the labelled states in this example have a 1:1 correspondence with
the `3` type:

- `small` = `0`
- `medium` = `1`
- `large` = `2`

The whole idea of ari is that binary data can be interpreted as one
giant number, and we provide ways to break big numbers down into
smaller numbers... and label them 🙂.

> **NOTE:** Many other languages can partially model the idea of sum
> types with "enums". The main issue though is enums are often
> "rounded" to the best matching primitive type(s) (usually some kind
> of int type). In ari every possible sum type has an equivalent
> natural type, which allows for a level of precision not possible in
> other type systems.

### Value bindings and dependent types

In ari all types have an implicit binding with a corresponding runtime
value. [Dereference expressions `@`](#dereference-expressions) can be
used on a type to bring this value from "value-space" into
"type-space".

Here's an example where they're used to describe a 24-bit RGB colour
image with a dynamically sized `width` & `height`:

```lisp
:byte (^ :bit 2 :bit-length 8)
:my-image-format
(*
  :width byte
  :height byte
  :image
  (^
    :pixel (* :r byte :g byte :b byte)
    @width
    @height
  )
)
```

In this example the `@width` and `@height` types are evaluated from
the runtime value of `width` & `height`.

Expressions are normally evaluated at compile time, but this pushes
the evaluation to runtime. Ari can use the same expressions between
compile time & runtime because it's [purely
functional](https://en.wikipedia.org/wiki/Functional_programming),
meaning that expressions don't depend on any hidden state.

### Labels are optional, but you can also label anything

Most languages force you to name things, even when the name is
redundant or unnecessary. A lot of these languages have to come up
with parallel constructs to match their named counterparts.
eg. `functions ↔ lambdas`, `structs ↔ tuples`... Often there isn't an
unnamed alternative. You'll be forced to decouple & name things even
when an inlined alternative would be easier to understand.

Ari takes a fundamentally different approach. Any expression can be
inlined without labelling, but also, all expressions can be labelled.

This gives developers flexibility to do whatever makes the most
sense. Maybe something is only used once and it would be simpler to
just inline it, maybe you don't even know how to name something yet,
but you want to define its structure. Maybe you just want to refer to
a type nested within another type without decoupling it from its
parent.

When you can label everything, anything can be a module.

## What ideas does ari steal?

Ari steals a bunch of good ideas from various places:

Academic Inspirations:

- [Type theory](https://en.wikipedia.org/wiki/Type_theory) &
  [algebraic types](https://en.wikipedia.org/wiki/Algebraic_data_type)
- [Group theory](https://en.wikipedia.org/wiki/Group_theory)
- [Set theory](https://en.wikipedia.org/wiki/Set_theory)
- [Curry-Howard
  correspondence](https://en.wikipedia.org/wiki/Curry%E2%80%93Howard_correspondence)
- [Arithmetic coding](https://en.wikipedia.org/wiki/Arithmetic_coding)
- [Purely functional
  programming](https://en.wikipedia.org/wiki/Purely_functional_programming)
- [Lazy evaluation](https://en.wikipedia.org/wiki/Lazy_evaluation)
- [Currying](https://en.wikipedia.org/wiki/Currying)
- [Homoiconicity](https://en.wikipedia.org/wiki/Homoiconicity)

Language Inspirations:

- [Lisp](<https://en.wikipedia.org/wiki/Lisp_(programming_language)>)
  ([s-expressions](https://en.wikipedia.org/wiki/S-expression) - ari
  is not a lisp, homoiconicity, functional-focused)
- [Rust](https://www.rust-lang.org) (sum types & other algebraic
  types)
- [Nix](https://nixos.org/manual/nix/stable/expressions/expression-language.html)
  (purely functional, lazily evaluated, currying)
- [Haskell](https://www.haskell.org) (indirectly through Rust & Nix +
  monads)

## Language reference

### Atomic expressions

These are the predefined base expressions of ari, they produce the
"atoms" for [symbolic expressions](#symbolic-expressions).

#### Natural expressions

```lisp
0 1 2 3 ...
```

Produce the [primitive
types](https://en.wikipedia.org/wiki/Primitive_data_type) of the
language, modelled after [natural
numbers](https://en.wikipedia.org/wiki/Natural_number). Its "value"
encodes the number of possible states that can exist in the type.

##### Bottom expression

```lisp
0
```

Produces the [bottom type
`⊥`](https://en.wikipedia.org/wiki/Bottom_type), which has no possible
states. You can think of this as the "error type".

This is the [identity
type](https://en.wikipedia.org/wiki/Identity_element) for [sum
expressions](#add-and-sum-expressions):

```lisp
(=
  (+ x 0)
  x
)
```

There are certain contexts that propagate the bottom type:

- [Product expressions](#multiply-and-product-expressions)
- The base of [map expressions](#exponentiate-and-map-expressions)

This "error propagation" can be "caught" by other expressions. For
example:

```lisp
(+ 256 (* 256 0))
```

`(* 256 0)` evaluates to `0`, the bottom type, but `(+ 256 0)`
evaluates to `256`... which is not the bottom type. This [sum
expression](#add-and-sum-expressions) catches the error!

This doesn't _seem_ useful when only working with natural expressions,
why not just remove expressions that propagate the bottom type?
Well.... types can also be evaluated at runtime with [dereference
expressions `@`](#dereference-expressions), and we use this same idea
to catch runtime errors.

##### Unit expression

```lisp
1
```

Produces the [unit type](https://en.wikipedia.org/wiki/Unit_type),
which has only one possible state. You can think of this as a type
that doesn't actually store any information.

This is the [identity
type](https://en.wikipedia.org/wiki/Identity_element) for [product
expressions](#add-and-sum-expressions):

```lisp
(=
  (* x 1)
  x
)
```

#### Label expressions

```lisp
:label 123
```

Define a name for the result of an expression in the current scope.

#### Symbol expressions

```lisp
symbol
```

Produce "symbol types", which are equivalent to whatever type is
associated with `symbol` in the current scope.

#### Value expressions

In ari, all types are implicitly bound to a corresponding runtime
value. Value expressions are used to refer to these runtime values.

They can only be used in:

- [Product expressions](#multiply-and-product-expressions)
- The base of [map expressions](#exponentiate-and-map-expressions)

> **NOTE:** These are the same contexts that propagate the [bottom
> type `0`](#bottom-expression).

##### Reference expressions

```lisp
$symbol
```

Produce "reference types", which are bound to the same runtime value
of `symbol`, but are equivalent to [`1`](#unit-expression).

##### Dereference expressions

```lisp
@symbol
```

When applied to symbol types, produce a type from the runtime value of
the symbol type.

```lisp
@123
@(+ :small 1 :medium 1 :large)
```

When applied to non-symbol types, produce an "effective type" from the
runtime value of the non-symbol type.

"Effective types" don't have an associated runtime value.

#### Path expressions

```lisp
symbol:x:y:z
(* :x (* :y (* :z 256))):x:y:z
```

Produce a nested type contained within another type.

#### Extended symbol expressions

```ari
'(symbol)'
```

Produce symbols that can include non-whitespace special characters.

To encode quotes in this expression, double them up:

```ari
'''quoted'''
```

> **NOTE:** To get a better understating of how this works, these
> expressions only terminate after an odd sequence of quotes. Then all
> terminating quotes (except for the last) are interpreted as part of
> the symbol.

Any characters following whitespace will be treated as documentation:

```lisp
:'pixel A 24bit RGB pixel'
(* :r 256 :g 256 :b 256)

'pixel Here we're referencing pixel'
```

#### Unicode text expressions

```ari
"()"
```

A convenient notation for defining text-based grammars, which is
actually just syntax sugar for runtime [assertion
expressions](#assertion-expressions) that assert for
[Unicode](https://en.wikipedia.org/wiki/Unicode) text.

To encode quotes in this expression, double them up:

```ari
"{
  ""abc"": 123
}"
```

> **NOTE** This follows the same behaviour as [extended symbol
> expressions](extended-symbol-expressions)

Text not bound to any particular encoding context will be interpreted
as utf-8, but this can be changed with text encoding macros:

```lisp
(=
  (ascii-8 "text")
  (ascii "text")
)
(ascii-7 "text")
(utf-16 "text")
(utf-32 "text")
```

##### Codepoint symbol

```lisp
codepoint
```

Represents a single Unicode [code
point](https://en.wikipedia.org/wiki/Code_point) for text in the
current text encoding context. This can be dynamically sized.

##### Grapheme symbol

```lisp
grapheme
```

Represents a single Unicode
[grapheme](https://en.wikipedia.org/wiki/Grapheme) for text in the
current text encoding context. This can be dynamically sized.

### Symbolic expressions

```lisp
(sexpr arg1 arg2 arg3)
```

Symbolic expressions are a list of [atomic
expressions](#atomic-expressions), separated by whitespace & wrapped
by parenthesis.

The first element is treated as a function, and the remaining elements
are passed as inputs to that function.

These are considered "symbolic expressions" because their behaviour
depends on the symbols defined in the current scope.

These modelled after lisp
[s-expressions](https://en.wikipedia.org/wiki/S-expression), but are
different in a few ways:

- Ari doesn't have [cons cells](https://en.wikipedia.org/wiki/Cons),
  so symbolic expressions aren't implemented with cons cells. The
  closest concept to cons cells are [product
  expressions](#multiply-and-product-expressions).

- Symbolic expressions form a lexical scope from the [label
  expressions](#label-expressions) it contains. This means that "let"
  expressions are embedded in all symbolic expressions.

#### Assertion expressions

```lisp
:equal (= a b c)
```

[Asserts](<https://en.wikipedia.org/wiki/Assertion_(software_development)>)
that all arguments have the same number of possible states. If they
do, this evaluates to the last argument, otherwise it evaluates to
[`0`](#bottom-expression).

<!-- ### Identity assertion expression -->

<!-- TODO: We need some way to do identity assertions to support type checking -->

#### Algebraic expressions

##### Additive expressions

###### Add and sum expressions

```lisp
:add (+ a b)
```

Given natural inputs, produces a [sum type
`∑`](https://en.wikipedia.org/wiki/Tagged_union), which is either `a`
or `b`. This adds together the number of possible states between the
two types.

We derive a nary form of add `+` by repeated application of
associativity:

```lisp
:additive-associativity
(=
  (+ (+ a b) c)
  (+ a (+ b c)
  (+ a b c)
)
```

###### Additive inverse expressions

```lisp
:subtract (- a b)
```

Produces an [integer type `ℤ`](https://en.wikipedia.org/wiki/Integer)
that subtracts the first few `b` states from `a`.

Combined with the [additive identity `0`](#bottom-expression), we
derive a unary form of subtract `-`:

```lisp
:negative
(=
  (- 0 x)
  (- x)
)
```

###### Additive group theory

Additive expressions form an [abelian
group](https://en.wikipedia.org/wiki/Abelian_group) in type-space, and
a [non-abelian](https://en.wikipedia.org/wiki/Non-abelian_group) group
in value-space.

##### Multiplicative expressions

###### Multiply and product expressions

```lisp
:multiply (* a b)
```

Given natural inputs, produces a [product type
`Π`](https://en.wikipedia.org/wiki/Product_type), which has both `a`
and `b`. This multiplies the number of possible states between the two
types.

We derive a nary form of multiply `*` by repeated application of
associativity:

```lisp
:multiplicative-associativity
(=
  (* (* a b) c)
  (* a (* b c))
  (* a b c)
)
```

###### Multiplicative inverse expressions

```lisp
:divide (/ a b)
```

Produces a [rational type
`ℚ`](https://en.wikipedia.org/wiki/Rational_number) type that divides
`b` states out of `a`.

Combined with the [multiplicative identity `1`](#unit-expression), we
derive a unary form of divide `/`:

```lisp
:inverse
(=
  (/ 1 x)
  (/ x)
)
```

###### Multiplicative group theory

Multiplicative expressions form an [abelian
group](https://en.wikipedia.org/wiki/Abelian_group) in type-space, and
a [non-abelian](https://en.wikipedia.org/wiki/Non-abelian_group) group
in value-space.

##### Exponentiation expressions

###### Exponentiate and map expressions

```lisp
:exponentiate (^ b e)
```

Given natural inputs, produces a [map type
`→`](https://en.wikipedia.org/wiki/Associative_array), which maps the
exponent `e` to the base `b`. This raises the number of possible
states in `b` to the power of the number of possible states in `e`.

Exponentiation is non-associative, but we derive a nary form of
exponentiate `^` through
[currying](https://en.wikipedia.org/wiki/Currying). Which is just
repeated application of the [power of a power
identity](https://en.wikipedia.org/wiki/Exponentiation#Identities_and_properties):

```lisp
:power-of-a-power-identity
(=
  (^ (^ a b) c)
  (^ a (* b c))
  (^ a b c)
)
```

###### Exponentiation left inverse expression

```lisp
:logarithm (log b x)
```

Produces a [complex type
ℂ](https://en.wikipedia.org/wiki/Complex_number), which treats `x`
like a map type (even if it's not), and computes the exponent `e`
given the base `b`.

###### Exponentiation right inverse expression

```lisp
:root (root x e)
```

Produces a [complex type
ℂ](https://en.wikipedia.org/wiki/Complex_number), which treats `x`
like a map type (even if it's not), and computes the base `b` given
the exponent `e`.

`root` is actually just syntax sugar for exponentiation with a
multiplicative inverse:

```lisp
:root
(=
  (^ x (/ e))
  (root x e)
)
```

###### Exponentiation group theory

Exponentiation expressions form a
[quasigroup](https://en.wikipedia.org/wiki/Quasigroup) in both
type-space & value-space.

#### Set expressions

Sets let you interpret a single value as multiple different
types. This is particularly useful when you don't actually know what
the type is, but the type can be deduced by its contents.

##### Singletons

All types are implicitly treated as
[singletons](<https://en.wikipedia.org/wiki/Singleton_(mathematics)>). These
are sets with exactly one type.

##### Empty set expression

```lisp
:empty-set ()
```

Produces the [empty set](https://en.wikipedia.org/wiki/Empty_set). A
set with no types.

##### Top expression

```lisp
:top-type _
```

Produces what is called a [top type
`⊤`](https://en.wikipedia.org/wiki/Top_type), but it's not exactly a
"type". It's a set containing every possible type.

`_` is actually just syntax sugar for any type that directly
references itself:

```lisp
:_ _
```

##### Union expressions

```lisp
:union (| a b)
```

Produces a set which takes the
[union](<https://en.wikipedia.org/wiki/Union_(set_theory)>) between `a`
and `b`.

We derive an nary form of union `|` by repeated application of
associativity:

```lisp
:union-associativity
(=
  (| (| a b) c)
  (| a (| b c)
  (| a b c)
)
```

###### Zero or one expression

```lisp
?
```

Syntax sugar for the [union `|`](#union-expressions) between
[`0`](#bottom-expression) and [`1`](#unit-expression):

```lisp
:? (| 0 1)
```

##### Symmetric difference expression

```lisp
:symmetric-difference (~ a b)
```

Produces a set which takes the [symmetric
difference](https://en.wikipedia.org/wiki/Symmetric_difference)
between `a` and `b`.

symmetric-difference `~` is actually just syntax sugar for the union
`|` of both relative complements `!`:

```lisp
(=
  (| (! a b) (! b a))
  (~ a b)
)
```

##### Intersection expressions

```lisp
:intersection (& a b)
```

Produces a set which takes the
[intersection](<https://en.wikipedia.org/wiki/Intersection_(set_theory)>)
between `a` and `b`.

We derive an nary form of intersection `&` by repeated application of
associativity:

```lisp
:intersection-associativity
(=
  (& (& a b) c)
  (& a (& b c)
  (& a b c)
)
```

##### Complement expressions

```lisp
:relative-complement (! a b)
```

Produces a set which takes the [relative
complement](<https://en.wikipedia.org/wiki/Complement_(set_theory)#Relative_complement>)
of `a` in `b`.

Combined with the top type `_`, we derive a unary form of
relative-complement `!`:

```lisp
:complement
(=
  (! x _)
  (! x)
)
```

###### One or more expression

```lisp
:one-or-more
(=
  (! 0)
  !)
```

Syntax sugar for the [complement `!`](#complement-expressions) of
[`0`](#bottom-expression):

##### Interval expression

```lisp
:interval (.. a b)
```

Produces a half-open
[interval](<https://en.wikipedia.org/wiki/Interval_(mathematics)>)
between `a` & `b`, where `b` is excluded.

Combined with the empty set `()`, we derive a unary form of interval
`..`:

```lisp
(=
  (.. () b)
  (.. b)
)
```

<!-- ## Scoping -->

<!-- (scope) -->

<!-- Self referencing has to be possible, how can we refer to something with the same name in the parent scope? -->

## License

Copyright (C) 2023 Kira Bruneau

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at
your option) any later version.

This program is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

### Why is this licensed under the GPL instead of the LGPL?

This was a difficult decision. By licensing this under the GPL, not
only are we preventing proprietary software from being combined with
ari, but also open source software using a license that's incompatible
with the GPL.

This restriction may _seem_ to go against our goal of making binary
data more open & accessible, but it very much works in favour of it.

Proprietary software is designed to restrict the freedoms of its users
for profit. By allowing ari to be used in proprietary software, we'd
be perpetuating the idea that software should be hard to access &
modify. This is exactly what we don't want.

This doesn't mean that users are restricted from using ari along side
proprietary software. The GPL is designed in a way to take away
freedoms from developers, and give as many freedoms as possible back
to users.

This license just requires that proprietary (or open source software
with a license allowing for integration with proprietary software)
can't be built on top of, or packaged together with ari.

See [why you shouldn't use the Lesser GPL for your next
library](https://www.gnu.org/licenses/why-not-lgpl.html).

<!-- What does it mean to call a type? -->
<!-- - Does it mean to project a subset? -->
<!-- - Does it mean to map from one type to another? -->
<!-- - Does it mean to cast one type to another? -->
