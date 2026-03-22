# syntax

let a = true;
Creates data.

res a = true;
Creates a resource.

The "@" symbol represents macros, which cannot be user-defined.

@some_macro(...)
@some_other_macro {}

@(10, 20)
Creates a tuple.

@{ "a": "hello", "b": "world" }
Creates a dictionary.

let one = @{ "money": "ties" };
let two = @{ "hello", "world", ..one };
Use .. to expand.

@[0, 1, 2, 3]
Creates a list.

let one = @[0, 1, 2, 3, 4];
let two = @[-1, -2, ..one];
Use .. to expand.

@derive(Clone)
struct Money(Int);
Derive specific behaviors. For example, upon deriving Clone, it makes it possible for the compiler to know that things inside should be cloned (if possible) on write, making it possible to be used as data.

{
  // ... statements/expressions here
}
A scope.

// a comment
/* also a comment */

{
  res a = "hello";
} // resource a definitely drops here

{
  let a = "hello";
} // data a might/might not drop, depending on its references
// a is unreachable here, but it might still be alive

let a_ref = &a;
Create a reference (alias/borrow/view) to 'a'

struct Cat {
  friend: Dog
}
Struct with fields.

struct Cat(Dog);
Struct with one field.

struct Cat(Dog, Dog);
Struct with multiple fields.

struct Cat;
Struct with nothing.

enum Animal {
  Dog,
  Cat
}
enum Animal {
  Dog(Dog),
  Cat(Cat)
}
Enums.

fn main() {

}
A function returning @().

fn foo() -> Bar {
  Bar("hello")
}
A function returning type "Bar"

fn borrow_it(food: &Food) {
  food.eat();
}
Function with a parameter taking data view.

fn maybe_borrow_it(food: Food) {
  food.eat();
}
Function with a parameter taking resource. It could be data, as data is resource under the hood.

match something {
  Stuff::A => {
    do_something();
    then_return_this()
  },
  Stuff::B => return_this(),
  _ => wildcard(),
}
Pattern matching.

if let Stuff::A(data) = stuff { /* ... */ }
while let Stuff::A(data) = stuff { /* ... */ }
Pattern matching control flows.

might_fail()?;
maybe_none()?;
Fallible/maybe option.

let a = 100;
let closure = || {
  a += 10;
};
Closures.

res a = 100;
let closure = move || {
  a += 10;
};
Closures with "move" keyword for resources.

#T
Unsafe pointer of type T (which is *T in C).

***

No type templates for now.
