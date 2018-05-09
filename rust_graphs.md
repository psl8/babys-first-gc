# A Note on Graph Structures in Rust

A garbage collected heap is, at its core, simply a graph of objects. As a result, the safe GC
in this repo uses a common idiom for constructing graphs in Rust, allocating in a vector and 
using indices as pointers. However, I'd like to clarify something: this idiom is *not* a hack.

Graph structures are something many programmers new to Rust struggle with. In fact, many people
have gotten the incorrect impression that graphs are hard to write in Rust. This is really not
the case. Seriously, go look at the code in `safe_gc.rs`. It's not rocket science. But it requires
that you know the trick of using indices. But *why* do we construct graphs like this? And *why* do
new Rustaceans struggle with graphs? The reason is simple: graph structures have nuanced ownership
semantics. The key is realizing that a graph owns all of its nodes, wholly and exclusively. Consider
a common (incorrect) way of writing a doubly-linked list in Rust:

```rust
struct Node<T> {
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
    data: T,
}
```

If you try to use this list, you'll quickly find that it doesn't work at all. The reason is ultimately
just because `Box<T>` takes ownership and nodes shouldn't own other nodes. The graph should own the 
nodes. A common thought is, "Well, I could *share* ownership using `Rc<T>` or `Arc<T>`". This scheme 
[can work][4], but it misses the point. We don't want to own *some nodes*, we want to own *a graph*. 
Sharing ownership in a graph causes the ownership relations themselves to become a graph. If this 
sounds overcomplicated, that's because it is. Things like `Rc<T>` exist to give you shared ownership, 
but we don't actually have more than one owner here.

[4]: http://cglab.ca/~abeinges/blah/too-many-lists/book/fourth.html

This is why graph structures often use `Vec` in Rust, because a vector owns all of its elements.
Additionally and crucially, an index is a form of reference that doesn't own its referent. This 
works because you need to own *the structure* to access a value through an index. You can't access 
any of the elements of a `Vec` unless you have access to the `Vec` itself (or have been given a 
reference by someone who does). This is important because no other form of reference in Rust has 
this property. Raw pointers (and `NonNull<T>` pointers) also don't own their referents, but
they *don't* enforce that you actually own the graph when you dereference them. Thus, *a raw 
pointer is the wrong kind of reference for a single-owner graph* because you have to manually 
enforce its invariants when you could get them automatically by using indices.

Of course, these are characteristics common to many kinds of collection, so really `Vec` gets
used because it's generally a Rust programmer's default collection and it works fine. Using,
say, `HashMap` or `VecDeque` or your own collection also works, so long as the collection
completely owns its elements and can be indexed.

Looking at [petgraph][6], a popular library for writing graphs in Rust, we can see that its
API matches exactly what we'd expect. `Node`s and `Edge`s are owned by the `Graph`, and
accesses happen *through the graph*, using indices. In fact, the `Graph` structure itself is
[implemented with vectors][7].

[6]: https://docs.rs/petgraph/0.4.12/petgraph/
[7]: https://docs.rs/petgraph/0.4.12/src/petgraph/graph_impl/mod.rs.html#325-329

