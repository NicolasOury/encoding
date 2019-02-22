A small set of macro and library to generate one-hot encoding for any types. 

The library consist of one trait, Encoding, containing 4 functions:
- encoding_size giving the size of the encoding for the type
- encode_into takes a mutable slice and encode a value into it
- encode is similar but create a new vector, its default implementation is probably always good
- likelihood, takes an encoding and give the naive likelihood of the value (naive in the sense of not expertly written not with a precise naive Bayes-like meaning)

This can be seen in the encoding library.

We build a set of macros to derive this for any finite types (see derive_encoding directory).

We provide a sketch of an example in example.