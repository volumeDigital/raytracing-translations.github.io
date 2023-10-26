# Translation Goal

1. Aim to directly substitute the implementation code. In this case, from C++ -> Rust.
2. The book is written from the first person. We will politely wait for a break before adding our comments.
3. The reader knows that the book's code was originally in C++, but they won't have to know any C++ to understand our translation.
4. Our changes will do one of the following:
   1. Add a note admonition to succinctly explain how we will deviate from the book.
   2. Add a warning admonition to explain why we've removed any content.
   3. Replace C++ code blocks with the most direct equivalent in valid Rust.
   4. Replace the words of the text that refer to directly to the code provided.

# Translation Rules




1. Don't get cute. Conform, where possible, to the original author's mental model of how the software should be expressed. If he'd disagree with your choice, he's right, and you're wrong, probably :)
3. Code style, layout: Imagine that Peter actually chose to write the book in (English/Rust). Would he be adding rando unit tests etc?
4. Be careful when removing/editing content. Imagine that the reader knows the original book is in (English/C++), and will be expecting the translator's voice to enter at an appropriate moment.
5. Consider the principle of least surprise, where we're managing Peter's surprise upon seeing the Rust translation. Software is the land of a million opinions. Keep it minimal, keep it respectful.
6. Be explicit about choices and rationale.
7. Be humble.
8. An editor/translator is _not_ an author. Write little to no new material.
9. Performance is not a priority. Accuracy of translation is. If there's a 'faster way' in Rust that doesn't look like the C++ code, don't do it.
10. Deviations should have strong rationale behind them.
11. Peter's voice goes first.


2. Operator Oveloading: Follow Rust guidelines wrt return type, including expectations regarding ref/copy.
3. Argument names: Literal translation of source document.
4. Function ordering: as for source material. Any methods that are deliverd by implementing a trait (std::ops::*, for example) can be broken out below the core impl block.
5. Small primitive should derive copy and clone. Rust requires software to explicit about conversion of some type T to/from a reference to T (aka &T). This clutters the code that uses these primitives with excessive dereferencing. Maybe implementing Deref could help with this? 
6. Comments: copy them directly.
7. Variable naming: as for C++.
8. Provide test coverage wherever possible. This proves that the included code is ok to copy/paste.