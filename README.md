# arie

A type-centred [purely functional
programming](https://en.wikipedia.org/wiki/Purely_functional_programming)
language designed to type binary files.

# Why does this exist?

## To make binary files more accessible

Most binary files require specially designed tools to read & write,
but arie types are designed so that you can decompose any binary file
down into its component parts. The language is intended to be a
foundation for other tools to build on top of. Some of the tools we
plan on building with arie include:

- [ ] A tool to query values in a binary file using arie types
- [ ] An editor specially designed to edit binary files using arie types
  - Including plugins to existing text editors
- [ ] A diff tool that structurally compares files of the same type
- [ ] A compression tool that compresses binary data with arie types
  - Arie's type system is designed to be a guide for [arithmetic
    coding](https://en.wikipedia.org/wiki/Arithmetic_coding) - it's
    partially where the name comes from (**ari**thmetic
    **e**ncoding/**e**ditor)

## ... but also, not just binary formats

While the primary focus is to help interpret binary data, arie is also
designed to model grammars for text-based languages.

Here is a quick comparison of how some formal language concepts map
into arie:

> **NOTE:** This comparison is meant for people who are already
> familiar with these concepts.
>
> If you want to get a better understanding of what they mean, or what
> the arie equivalents mean, see the [language
> reference](#language-reference).

<table>
<tr>

<td>
<table>
<tr><th>Regex</th><th>Arie</th></tr>

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
(^ "a" .)
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
<tr><th>Backus–Naur form</th><th>Arie</th></tr>

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

# What makes arie unique?

## Types are the main focus of the language

In arie, everything is a type. Types and type expressions are the
primary focus of the language. You can add, multiply, and even
exponentiate types, much like you can a number in any other
programming language.

<table>
<tr><th>Arie type expressions</th><th>Equivalent types</th></tr>

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
  tuple](https://en.wikipedia.org/wiki/Record_(computer_science))
- [class](https://en.wikipedia.org/wiki/Class_(computer_programming))
- [trait](https://en.wikipedia.org/wiki/Trait_(computer_programming))
- [concept,
  interface](https://en.wikipedia.org/wiki/Concept_(generic_programming))
- [protocol](https://en.wikipedia.org/wiki/Protocol_(object-oriented_programming))

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
- [matrix](https://en.wikipedia.org/wiki/Matrix_(mathematics))
- [vector space](https://en.wikipedia.org/wiki/Vector_space)
- [tensor](https://en.wikipedia.org/wiki/Tensor)

</td>
</tr>
</table>

See the [language reference](#language-reference)

## Generalizes the idea of primitive types

In arie, there are infinitely many primitives types generalized by a
single concept, [natural
numbers](https://en.wikipedia.org/wiki/Natural_number). We call these
primitive types [natural types](#natural-expressions).

...well what exactly does this mean? Say you want to create a `size`
type that has 3 possible states, `small`, `medium`, `large`. In arie,
you can precisely type the number of states using the `3` type:

```lisp
:size 3
```

The possible values of this type are `0`, `1`, `2`, which you can use
to represent `small`, `medium`, `large`. This is a powerful concept,
but this small example doesn't fit with the main goal of arie, to make
things more accessible.

...this is exactly where algebraic expressions & labels come in! You
can break down and label the individual states of a `3` type with a
[sum expression](#additive-expressions). For example, this sum
expression produces a sum type that "sums" to the same number of
states as the `3` type:

```lisp
:size (+ :small 1 :medium 1 :large 1)
```

and the labelled states in this example have a 1:1 correspondence with
the `3` type:

- `small` = `0`
- `medium` = `1`
- `large` = `2`

The whole idea of arie is that binary data can be interpreted as one
giant number, and we provide ways to break big numbers down into
smaller numbers... and label them 🙂.

> **NOTE:** Many other languages can partially model the idea of sum
> types with "enums". The main issue though is enums are often
> "rounded" to the best matching primitive type(s) (usually some kind
> of int type). In arie every possible sum type has an equivalent
> natural type, which allows for a level of precision not possible in
> other type systems.

## Value bindings & dependant types

In arie all types have an implicit binding with a corresponding
runtime value. [`@`](#dereference-expression) can be used on a type to
bring this implicitly bound value from "value-space" into
"type-space".

Here's an example where [`@`](#dereference-expression) is used to help
describe an 24-bit RGB colour image will a dynamically sized `width` &
`height`:

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

This concept, along with the fact that arie is [purely
functional](https://en.wikipedia.org/wiki/Functional_programming),
blurs the line between compile time and runtime. Expressions are
usually evaluated at compile time, unless they contain a type
dereferenced with [`@`](#dereference-expression).

## Labels are optional, but you can also label anything

Most languages force you to name things, even when the name is
redundant or unnecessary. A lot of these languages have to come up
with parallel constructs to match their named counterparts.
eg. `functions ↔ lambdas`, `structs ↔ tuples`... Often there isn't an
unnamed alternative. You'll be forced to decouple & name things even
when an inlined alternative would be easier to understand.

Arie takes a fundamentally different approach. Any expression can be
inlined without labelling, but also, all expressions can be labelled.

This gives developers flexibility to do whatever makes the most
sense. Maybe something is only used once and it would be simpler to
just inline it, maybe you don't even know how to name something yet,
but you want to define its structure. Maybe you just want to refer to
a type nested within another type without decoupling it from its
parent.

When you can label everything, anything can be a module.

# What ideas does arie steal?

Arie steals a bunch of good ideas from various places:

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
- [Lisp](https://en.wikipedia.org/wiki/Lisp_(programming_language))
  ([s-expressions](https://en.wikipedia.org/wiki/S-expression) - arie
  is not a lisp, homoiconicity, functional-focused)
- [Rust](https://www.rust-lang.org) (sum types & other algebraic
  types)
- [Nix](https://nixos.org/manual/nix/stable/introduction.html) (purely
  functional, lazily evaluated, currying)
- [Haskell](https://www.haskell.org) (indirectly through Rust & Nix +
  monads)

# Language reference

## Syntactic expressions

Syntactic expressions are the base expressions of arie. They are
defined without parenthesis, in contrast to the [symbolic
expressions](https://en.wikipedia.org/wiki/S-expression).

Anything that's a syntactic expression is designed that way to reduce
the syntax noise of parenthesis.

### Natural expressions

```
0, 1, 2, 3, ...
```

Produce the [primitive
types](https://en.wikipedia.org/wiki/Primitive_data_type) of the
language, which are based on [natural
numbers](https://en.wikipedia.org/wiki/Natural_number). Its "value"
encodes the number of possible states that can exist in the type.

#### Bottom expression

```lisp
0
```

Produces the [bottom type
`⊥`](https://en.wikipedia.org/wiki/Bottom_type), which is essentially
an "error" type that has no states.

It can't be directly used by itself, in a [product
expression](#multiply-product-expressions), nor in the base of a [map
expression](#exponentiate-map-expressions) without being wrapped in
some other context. You can think of these bad contexts as expressions
that automatically propagate the error state, and other expressions as
contexts that stop the error propagation.

#### Unit expression

```lisp
1
```

Produces the [unit type](https://en.wikipedia.org/wiki/Unit_type),
which only has one possible state, and doesn't store any information.

### Label expressions

```lisp
:label 123
```

Gives a name to the result of an expression.

### Reference expressions

```lisp
label
```

References a type by its label.

It's automatically labelled with the same name if it's not wrapped by
a label expression.

### Dereference expressions

```lisp
@label
```

Brings the runtime value bound to `label` into type-space.

### Path expressions

```lisp
label:x:y:z
(* :x (* :y (* :z 256))):x:y:z
```

References a type nested within another type.

### Extended label expressions

```
'label'
''label''
'''label'''
...
```

A way to define and reference a label that includes special
characters. If you want to use the prefix/suffix quotes in the
expression, you can always add another quote to the prefix & suffix.

When used in label expressions, the first non-empty line will be
interpreted as the label, and the following lines will be interpreted
as the description:

```lisp
:''
  pixel
  A 24bit RGB pixel
''
(* :r 256 :g 256 :b 256)

pixel
```

> **NOTE:** This means its impossible to define a multiline label

### Unicode text expressions

```
"text"
""text""
"""text"""
...
```

Syntax sugar to [assert](#assertion-expressions) for
[Unicode](https://en.wikipedia.org/wiki/Unicode) text. This is used to
define text-based grammars. If you want to use the prefix/suffix
quotes in the expression, you can always add another quote to the
prefix & suffix.

Without an encoding context, text is interpreted as
[UTF-8](https://en.wikipedia.org/wiki/UTF-8), but can be interpreted
with a different encoding with text encoding macros:

```lisp
(=
  (ascii-8 "text")
  (ascii "text")
)
(ascii-7 "text")
(utf-16 "text")
(utf-32 "text")
```

Single-line text expressions are automatically labelled by their
contents if not wrapped by a label expression.

#### Codepoint reference expression

```lisp
codepoint
```

Represents a single Unicode [code
point](https://en.wikipedia.org/wiki/Code_point) for text in the
current text encoding context. This can be dynamically sized.

#### Grapheme reference expression

```lisp
grapheme
```

Represents a single Unicode
[grapheme](https://en.wikipedia.org/wiki/Grapheme) for text in the
current text encoding context. This can be dynamically sized.

## Assertion expressions

```lisp
:equal (= a b c)
```

[Asserts](https://en.wikipedia.org/wiki/Assertion_(software_development))
that all arguments have the same number of possible states. If they
do, this evaluates to the last argument, otherwise it evaluates to
[`0`](#bottom-expression).

<!-- ### Identity assertion expression -->

<!-- TODO: We need some way to do identity assertions to support type checking -->

## Additive expressions

### Additive identity

```lisp
:additive-identity 0
```

The [additive
identity](https://en.wikipedia.org/wiki/Additive_identity) is
[`0`](#bottom-expression).

### Add & sum expressions

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

### Additive inverse expressions

```lisp
:subtract (- a b)
```

Produces an [integer type `ℤ`](https://en.wikipedia.org/wiki/Integer)
that subtracts the first few `b` states from `a`.

Combined with the additive identity `0`, we derive a unary form of
subtract `-`:

```lisp
:negative
(=
  (- 0 x)
  (- x)
)
```

### Additive group theory

Additive expressions form an [abelian
group](https://en.wikipedia.org/wiki/Abelian_group) in type-space, and
a [non-abelian](https://en.wikipedia.org/wiki/Non-abelian_group) group
in value-space.

## Multiplicative expressions

### Multiplicative identity

```lisp
:multiplicative-identity 1
```

The [multiplicative
identity](https://en.wikipedia.org/wiki/Multiplicative_identity) is
[`1`](#unit-expression).

### Multiply & product expressions

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

### Multiplicative inverse expressions

```lisp
:divide (/ a b)
```

Produces a [rational type
`ℚ`](https://en.wikipedia.org/wiki/Rational_number) type that divides
`b` states out of `a`.

Combined with the multiplicative identity `1`, we derive a unary form
of divide `/`:

```lisp
:inverse
(=
  (/ 1 x)
  (/ x)
)
```

### Multiplicative group theory

Multiplicative expressions form an [abelian
group](https://en.wikipedia.org/wiki/Abelian_group) in type-space, and
a [non-abelian](https://en.wikipedia.org/wiki/Non-abelian_group) group
in value-space.

## Exponentiation expressions

### Exponentiate & map expressions

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

### Exponentiation left inverse expression

```lisp
:logarithm (log b x)
```

Produces a [complex type
ℂ](https://en.wikipedia.org/wiki/Complex_number), which treats `x`
like a map type (even if it's not), and computes the exponent `e`
given the base `b`.

### Exponentiation right inverse expression

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

### Exponentiation group theory

Exponentiation expressions form a
[quasigroup](https://en.wikipedia.org/wiki/Quasigroup) in both
type-space & value-space.

## Set expressions

Sets let you interpret a single value as multiple different
types. This is particularly useful when you don't actually know what
the type is, but the type can be deduced by its contents.

### Singletons

All types are implicitly treated as
[singletons](https://en.wikipedia.org/wiki/Singleton_(mathematics)). These
are sets with exactly one type.

### Empty set expression

```lisp
:empty-set ()
```

Produces the [empty set](https://en.wikipedia.org/wiki/Empty_set). A
set with no types.

#### Top expression

```lisp
:top-type .
```

Produces what is called a [top type
`⊤`](https://en.wikipedia.org/wiki/Top_type), but it's not exactly a
"type". It's a set containing every possible natural type
(eg. [ℕ](https://en.wikipedia.org/wiki/Natural_number)).

`.` is actually just syntax sugar for any type that directly
[references](#reference-expressions) itself:

```lisp
:. .
```

### Union expressions

```lisp
:union (| a b)
```

Produces a set which takes the
[union](https://en.wikipedia.org/wiki/Union_(set_theory)) between `a`
and `b`.

We derive an nary form of union `|` by repeated application of
associativity:

```lisp
:union-associativity
(|
  (| (| a b) c)
  (| a (| b c)
  (| a b c)
)
```

#### Zero or one expression

```lisp
?
```

Syntax sugar for the [union `|`](#union-expressions) between
[`0`](#bottom-expression) and [`1`](#unit-expression):

```lisp
:? (| 0 1)
```

### Symmetric difference expression

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

### Intersection expressions

```lisp
:intersection (& a b)
```

Produces a set which takes the
[intersection](https://en.wikipedia.org/wiki/Intersection_(set_theory))
between `a` and `b`.

We derive an nary form of intersection `&` by repeated application of
associativity:

```lisp
:intersection-associativity
(&
  (& (& a b) c)
  (& a (& b c)
  (& a b c)
)
```

### Complement expressions

```lisp
:relative-complement (! a b)
```

Produces a set which takes the [relative
complement](https://en.wikipedia.org/wiki/Complement_(set_theory)#Relative_complement)
of `a` in `b`.

Combined with the top type `.`, we derive a unary form of
relative-complement `!`:

```lisp
:complement
(=
  (! x .)
  (! x)
)
```

which can also be combined with [`0`](#bottom-expression) as syntax
sugar for the complement of [`0`](#bottom-expression):

#### One or more expression

```lisp
:one-or-more
(=
  (! 0)
  !)
```

Syntax sugar for the [complement `!`](#complement-expressions) of
[`0`](#bottom-expression):

### Interval expression

```lisp
:interval (.. a b)
```

Produces a half-open
[interval](https://en.wikipedia.org/wiki/Interval_(mathematics))
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

<!-- What does it mean to call a type? -->
<!-- - Does it mean to project a subset? -->
<!-- - Does it mean to map from one type to another? -->
<!-- - Does it mean to cast one type to another? -->
