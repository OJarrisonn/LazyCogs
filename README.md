# Lazy Cogs

A Rust library that implements simple lazy clonable data structures using the concepts of immutable data structures.

## What is lazy cloning?

Lazy cloning is a technich that lets one clone a data structure in O(1) time. It means that the amount of time needed to clone the data structure is constant and "doesn't depends on the amount of data held by the structure".

## How is it possible?

Reference counting. Instead of calling real `Clone::clone`, Lazy Cogs does `Rc::clone` behind the scenes (or `Arc::clone` for our `Atomic` variants). This works because we asume that the cloned structure will not be mutated. So we just throw references to the original data.

## Immutable means I can't change the data?

No. Immutable means that it's most likely to not be mutated. But in fact you can mutate the data. When data is mutated actual `Clone::clone`s are done. In our builtin implementations, we make sure that every attempt to mutate data doesn't affect any other clone.

That's the beautiful of laziness. Why should I take a huge amount of time cloning data that could be cheap references? I'll only clone when absolutely needed.

## Why use Lazy Cogs?

We provide a `LazyClone` trait and two lazy cloning wrappers `Lc` and `Alc` (thread-safe).
The trait is useful for defining yourself how your data structures can be lazily cloned. While the wrappers are used to wrap data that doesn't implements `LazyClone` inside your strutures.

It's worth to mention that data that implements `Copy` isn't worthy to wrap in a `Lc`, `Alc` nor implement `LazyClone`. Also if there's a struture that does the job and implements `LazyClone` it's recommended to use it instead of wrapping another that isn't `LazyClone`able with `Lc` or `Alc`.

## The `LazyClone` Trait

It's a trait that has three methods:

- `lazy`: produces a lazy clone of the data
- `eager`: produces a eager clone (regular clone) of the data. Useful for when you know that data is going to be mutated
- `is_mutable`: verifies if the structure can be mutated without affecting its lazy clones

The trait is implemented by `Lc`and `Alc`.

## Collections

Lazy Cogs also provides some out-of-the-box lazy implementations of collections. At the moment just `LazyVec` and `LazyList` (both with `Atomic` variants) which are respectively implementations of a `Vec` and a `LinkedList`. They aren't simple wrappers, they have some internal logic that makes them lazy.
