# arie

A type-centered [purely functional programming
language](https://en.wikipedia.org/wiki/Purely_functional_programming)
designed to type binary files.

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
    kind of where the name comes from (**ari**thmetic
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
char
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

- [record, struct, tuple](https://en.wikipedia.org/wiki/Record_(computer_science))
- [class](https://en.wikipedia.org/wiki/Class_(computer_programming))
- [trait](https://en.wikipedia.org/wiki/Trait_(computer_programming))
- [concept, interface](https://en.wikipedia.org/wiki/Concept_(generic_programming))
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

- [function type signature](https://en.wikipedia.org/wiki/Type_signature)
- [associative array, map, dictionary](https://en.wikipedia.org/wiki/Associative_array)
- [array type](https://en.wikipedia.org/wiki/Array_data_type)
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
to represent `small`, `medium`, `large`. This is an incredibly
powerful concept, but this small example doesn't fit with the main
goal of arie, to make things more accessible.

...this is exactly where expressions & labels come in! You can break
down and label the individual states of a `3` type with a [sum
expression](#additive-expressions). For example, this sum expression
produces a sum type that "sums" to the same number of states as the
`3` type:

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
bring this implicitly bound value from _"value-space"_ into
_"type-space"_:

Here's an example where [`@`](#dereference-expression) is used to help
describe an 24-bit RGB color image will a dynamically sized `width` &
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
    @height))
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
parent. When you can label everything, anything can be a module.

One of the core philosophies of arie is to minimize friction, while
also providing a level of precision that isn't possible in other
languages.

# What ideas does arie steal?

Arie steals a bunch of good ideas from various places:

Academic Inspirations:
- [Type theory](https://en.wikipedia.org/wiki/Type_theory) & [algebraic types](https://en.wikipedia.org/wiki/Algebraic_data_type)
- [Group theory](https://en.wikipedia.org/wiki/Group_theory)
- [Set theory](https://en.wikipedia.org/wiki/Set_theory)
- [Arithmetic coding](https://en.wikipedia.org/wiki/Arithmetic_coding)
- [Purely functional programming](https://en.wikipedia.org/wiki/Purely_functional_programming)
- [Lazy evaluation](https://en.wikipedia.org/wiki/Lazy_evaluation)
- [Currying](https://en.wikipedia.org/wiki/Currying)
- [Homoiconicity](https://en.wikipedia.org/wiki/Homoiconicity)

Language Inspirations:
- [Lisp](https://en.wikipedia.org/wiki/Lisp_(programming_language)) ([s-expressions](https://en.wikipedia.org/wiki/S-expression) - arie is not a lisp, homoiconicity, functional-focused)
- [Rust](https://www.rust-lang.org) (sum types & other algebraic types)
- [Nix](https://nixos.org/manual/nix/stable/introduction.html) (purely functional, lazily evaluated, currying)
- [Haskell](https://www.haskell.org) (indirectly through Rust & Nix + monads)

# Language reference

## Natural expressions

```
0, 1, 2, 3, ...
```

Produce the [primitive
types](https://en.wikipedia.org/wiki/Primitive_data_type) of the
language, which are based on [natural
numbers](https://en.wikipedia.org/wiki/Natural_number). Its "value"
encodes the number of possible states that can exist in the type.

### Bottom expression

```lisp
:bottom-type 0
```

Produces a [bottom type `⊥`](https://en.wikipedia.org/wiki/Bottom_type),
which is essentially an "error" type that has no states.

It can't be directly used by itself, in a [product
expression](#multiplicative-expressions), nor in the base of a
[map expression](#exponentiation-expressions) without being
wrapped in some other context. You can think of these bad contexts as
expressions that automatically propagate the error state, and other
expressions as contexts that stop the error propagation.

Its main purpose is to reduce a [type set](#set-expressions), when a
type in the set evaluates to `0` at runtime.

### Unit expression

```lisp
:unit-type 1
```

Produces a [unit type](https://en.wikipedia.org/wiki/Unit_type), which
only has one possible state, and therefore doesn't store any
information.

### Top expression

```lisp
:top-type .
```

Produces a [top type `⊤`](https://en.wikipedia.org/wiki/Top_type).

A top type is not quite a natural type... it's better to think of it
as a [type superset](#set-expressions) that represents every possible
natural type (eg. [ℕ](https://en.wikipedia.org/wiki/Natural_number)).

It's actually just [syntax sugar](#top-expression-sugar) for any type that
directly references itself.

<!-- #### Lazy top expression -->

<!-- TODO: Top expressions are greedy by default, we also need a lazy top -->
<!-- expression. Is there some way we can generalize this idea, or is it -->
<!-- just a hard-coded special case of a top-type? -->

## Assertion expression

```lisp
:assert (= a b c)
```

Asserts that all arguments have the same number of possible states. If
they do, this expressions evaluates to the last argument, otherwise it
evaluates to [`0`](#bottom-expression).

<!-- ### Identity assertion type -->

<!-- TODO: We need some way to do identity assertions to support type checking -->

## Additive expressions

```lisp
:add (+ a b)
```

Produces a [sum type `∑`](https://en.wikipedia.org/wiki/Tagged_union)
between two natural expressions, which is either `a` or `b`. This adds
together the number of possible states between the two types.

```lisp
:subtract (- a b)
```

Produces an [integer type `ℤ`](https://en.wikipedia.org/wiki/Integer)
that subtracts the first few `b` states from `a`.

`add` expressions are generalized into `sum` expressions through
associativity. See [sum sugar](#sum-sugar).

A "unary inverse" is defined by the [identity
`0`](#bottom-expression), and the binary inverse `-`. See [additive
inverse sugar](#additive-inverse-sugar).

> Additive expressions form an [albeian
> group](https://en.wikipedia.org/wiki/Abelian_group) in type-space,
> and a [non-albeian](https://en.wikipedia.org/wiki/Non-abelian_group)
> group in value-space.

## Multiplicative expressions

```lisp
:multiply (* a b)
```

Produces a [product type
`Π`](https://en.wikipedia.org/wiki/Product_type) between two natural
expressions, which has both `a` and `b`. This multiplies the number of
possible states between the two types.

```lisp
:divide (/ a b)
```

Produces a [rational type
`ℚ`](https://en.wikipedia.org/wiki/Rational_number) type that divides
`b` states out of `a`.

`multiply` expressions are generalized into `product` expressions
through associativity. See [product sugar](#product-sugar).

A "unary inverse" is defined by the [identity `1`](#unit-expression),
and the binary inverse `/`. See [multiplicative inverse
sugar](#multiplicative-inverse-sugar).

> Multiplicative expressions form an [albeian
> group](https://en.wikipedia.org/wiki/Abelian_group) in type-space,
> and a [non-albeian](https://en.wikipedia.org/wiki/Non-abelian_group)
> group in value-space.

## Exponentiation expressions

```lisp
:exponentiate (^ b e)
```

Produces a [map type
`→`](https://en.wikipedia.org/wiki/Associative_array) between two
natural expressions, which map the exponent `e` to the base `b`. This
raises the number of possible states in `b` by the number of possible
states in `e`.

```lisp
:root (root a b)
```

<!-- TODO: description -->
<!-- ["complex" ℂ](https://en.wikipedia.org/wiki/Complex_number) -->

```lisp
:logarithm (log a b)
```

<!-- TODO: description, also should the argument order reversed? log_b a? -->

Exponentiation is non-associative, but `exponentiate` expressions are
generalized into `map` expressions through
[currying](https://en.wikipedia.org/wiki/Currying). See [map
sugar](#map-sugar).

> Exponentiation expressions form a
> [quasigroup](https://en.wikipedia.org/wiki/Quasigroup) in both
> type-space & value-space. `root` is considered the "left division"
> and `log` is considered the "right division".

# Set expressions

Set expressions are used to create type sets. Type sets let you
interpret a single value as multiple different types. This is
particularly useful when you don't actually know what the type is, but
the type can be deduced by its contents.

## Union expression

```lisp
:union (| a b)
```

## Difference expression

```lisp
:difference (\ a b)
```

# Intersection expression

```lisp
:intersection (& a b)
```

# Range expression

```lisp
:range (.. a b)
```

# Type expressions

<!-- wow cool! -->

# Symbolic expressions

Symbols are any sequence of characters that don't contain whitespace
` ` or parenthesis `()`*. Inside the context of a symbol, arie defines a
domain specific language called symbolic expressions.

> \* [Escape expressions](#escape-expression) can include spaces and
> parenthesis and are still treated as a single symbol

## Natural literals

```lisp
42
```

## Label expression

```lisp
:my-type 1
```

Gives a name to the result of an expression

## Reference expression

```lisp
:my-type 2

my-type
```

References another type by its label

## Path expression

```lisp
:my-type (* :x 256 (* :y 256  (* :z 256)))

my-type:x:y:z
```

References a type nested within another type:

## Dereference expression

```lisp
@symbol
```

Brings the runtime value associated with a type into type-space

## Escape expressions

### Symbol escape expression

```
'symbol'
''symbol''
'''symbol'''
...
```

### String escape expression

```
"string"
""string""
"""string"""
...
```

<!-- ## Scoping -->

<!-- (scope) -->

<!-- Self referencing has to be possible, how can we refer to something with the same name in the parent scope? -->

# Syntax Sugar

## Additive expression sugar

### Sum sugar

```lisp
:sum
(=
  (+ (+ a b) c)
  (+ a (+ b c)
  (+ a b c))
```

### Additive identity sugar

```lisp
:additive-identity
(=
  1
  (* x (/ x)))
```

### Additive inverse sugar

```lisp
:additive-inverse
(=
  (- 0 x)
  (- x))
```

## Multiplicative expression sugar

### Product sugar

```lisp
:product
(=
  (* (* a b) c)
  (* a (* b c))
  (* a b c))
```

### Multiplicative identity sugar

```lisp
:multiplicative-identity
(=
  1
  (* x (/ x)))
```

### Multiplicative inverse sugar

```lisp
:multiplicative-inverse
(=
  (/ 1 x)
  (/ x))
```

## Exponentiation expression sugar

### Map sugar

```lisp
:map
(=
  (^ a b c)
  (^ a (* b c)))
```

### Root sugar

```lisp
:root
(=
  (^ a (/ b))
  (root a b))
```

### Imaginary expression sugar

```lisp
:i (root -1 2)
```

## Top expression sugar

```lisp
:x x
:. x
```

## Zero or one sugar

```lisp
:? (| 0 1)
```

Syntax sugar for the [union `|`](#union) between
[`0`](#bottom-expression) and [`1`](#unit-expression).

## One or more sugar

```lisp
:! (\ . 0)
```

Syntax sugar for [`.`](#top-expression), without [`0`](#bottom-expression).

## Strings

```lisp
"string"
```

```lisp
char
```

```lisp
(utf-8 "string")
(utf-16 "string")
(utf-32 "string")
```

<!-- What does it mean to call a type? -->
<!-- - Does it mean to project a subset? -->
<!-- - Does it mean to map from one type to another? -->
<!-- - Does it mean to cast one type to another? -->
