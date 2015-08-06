======
Probor
======


:Status: Proof of Concept


Probor is a protocol on top of CBOR_ that provides protobuf_-like functionality.

In particular CBOR has following limitations:

* Transmitting lots of small objects is expensive because keys are encoded in
  each object
* No standard schema for serializing algebraic types
* Lack of type casting by default

And protobuf has the following limitations:

* Can't parse data if no schema known (or no code generated at your fingertips)
* Generated types don't look like native (i.e. type casting required anyway)
* Semantics of fields are somewhat strange (e.g. what is repeated field?
  why not an array?)

So what probor contains:

1. Schema definition language
2. A description how objects are stored in CBOR_ format
3. A description of backward/forward compatibility rules
4. (De)serialization library that doesn't rely on code generation
5. Optional code generator for scaffolding

For example, here is schema::

    struct SearchResults {
        total_results @0 :int
        results @1 :array Page
    }
    struct Page {
        url @0 :text,
        title @1 :text,
        snippet @2 :optional text,
    }

Note the following things:

* We use generic type names like int (integer), not fixed width (see FAQ)
* We give each field a number, they are similar to ones used in other
  IDL's (like protobuf or capnproto)

The structure serialized with protobor will look like (diplaying
json for clarity, in fact you will see exact this data if decode CBOR and
encode with JSON):

.. code-block:: json

   [1100, [
        ["http://example.com", "Example Com"],
        ["http://example.org", "Example Org", "Example organization"]]]

Obviously when unpacked, it looks more like (in javascript):

.. code-block:: javascript

   new SearchResults({"total_results": 1100,
                      "results": [new Page({"url": "http://example.com",
                                            "title": "Example Com"}),
                                  new Page({"url": "http://example.org",
                                            "title": "Example Org",
                                            "snippet": "Example organization"})]}

Actually the object can be serialized like this:

.. code-block:: json

   {"total_results": 1100,
    "results": [{"url": "http://example.com",
                 "title": "Example Com"},
                {"url": "http://example.org",
                 "title": "Example Org",
                 "snippet": "Example organization"}]}

And this would also be **totally valid** serialized representation. I.e. you
can store fields both by names and by numbers. This is ocasionally useful for
ad-hoc requests or you may be willing to receive non-compact data from frontend
then validate and push data in more compact format for storage.

In Python serialization looks like:

.. code-block:: python

    from probor import struct

    class Page(object):

        def __init__(self, url, title, snippet=None):
            # .. your constructor .. omitted for brewity

        probor_protocol = struct(
            required={(0, "url"): str, (1, "title"): str},
            optional={(2, "snippet"): str})

    class SearchResults(object):
        def __init__(self, total_resutls, results):
            # .. your constructor .. omitted for brewity

        probor_protocol = struct(
            required={(0, "total_results"): int, (1, "results"): Page})


TODO: isn't syntax ugly? Should it be more imperative? Is setstate/getstate
used?

.. note:: It's easy to build a more declarative layer on top of this protocol.
   I.e. for some ORM model, you might reuse field names and types. But the
   important property to keep in mind is that you should not rely on field
   order for numbering fields and **numbers must be explicit**, or otherwise
   removing a field might go unnoticed.

   Apart from that, integrating probor data types with model and/or validation
   code is encouraged. And that's actually a reason why we don't provide a
   nicer syntax for this low-level declarations.


Similarly in Rust it looks like:

.. code-block:: rust

    struct Page {
        url: String,
        title: String,
        snippet: Option<String>,
    }

    struct SearchResults {
        total_results: u64,
        results: Vec<Page>,
    }

    impl ProborEncode for Page {
        fn encode(&self, e) -> Result<(), EncodeError> {
            probor_enc_header!(e, 3, optional={2: self.snippet});
            probor_enc_field!(e, 0, "url", self.url, text);
            probor_enc_field!(e, 1, "title", self.title, text);
            probor_enc_field!(e, 2, "snippet", self.snippet, optional text);
        }
    }
    impl ProborDecode for Page {
        fn decode(&self, e) -> Result<(), DecodeError> {
            probor_dec_struct! {
                url (0) => d.text(),
                title (1) => d.text(),
                snippet (2) => d.text(),
            };
            probor_dec_require!(url, title);
            Ok(Page { url: url, title: title, snippet: snippet })
        }
    }
    impl ProborEncode for SearchResults {
        fn encode(&self, e) -> Result<(), EncodeError> {
            probor_header!(e, 3);
            probor_field!(e, 0, "total_results", self.total_results, u64);
            probor_field!(e, 1, "results", self.results, array Page);
        }
    }
    impl ProborDecode for SearchResults {
        fn decode(&self, e) -> Result<(), DecodeError> {
            probor_dec_struct! {
                total_results (0) => d.text(),
                results (1) => { d.decode_array_of(Page) },
            };
            probor_dec_require!(total_results, results);
            Ok(SearchResults { total_results: total_results,
                               results: results })
        }
    }

The rust code is a bit longer which is bearable for rust.  It's hugely based on
macros, which may seem as similar to code generation. Still we seem it better
because you are in control of at least the following things:

1. The specific types used (e.g. u64 for int)
2. The structure definition (may use meta attributes including
   ``derive`` and ``repr`` and may use ``struct T(X, Y)``)
3. How objects are created (e.g. use ``VecDeque`` or ``BTreeMap`` instead of
   default ``Vec`` and ``HashMap``)
4. How missing fields are handled. E.g. you can provide default for missing
   field instead of using ``Option<>``
5. Can include application specific validation code

At the end of the day writing parser explicitly with few helper macros looks
like much better idea than adding all the data as the meta information to the
schema file.


Type System
===========

Structures
----------

TBD

Algebraic Types
---------------

TBD

In Unsupported Languages
````````````````````````

In language which doesn't support algebraic types they are implemented
by tying together few normal types. E.g. the following type in rust:

.. code-block:: rust

    enum HtmlElement {
        Tag(String, Vec<HtmlElement>),
        Text(String),
    }

Is encoded like this in python:

.. code-block:: python

    from probor import enum

    class HtmlElement:
        """Base class"""

    class Tag(HtmlElement):
        def __init__(self, tag_name, children):
            # .. snip ..

        probor_protocol = ...

    class Text(HtmlElement):

        def __init__(self, text)
            self.text = text

        probor_protocol = ...


    HtmlElement.probor_protocol = enum({
        (0, 'Tag'): Tag,
        (1, 'Text'): Text,
        })

Then you can do pattern-matching-like things by using
``functools.singledispatch`` (in Python3.4) or just use ``isinstance``.

.. note:: The purescript compiles compiles types similarly. It's unchecked but
   I believe probor's searization into javascript, should be compatible with
   purescript types.


Forward/Backward Compatibility
==============================

Comparing with protobuf the probor serializer always considers all fields as
optional. The required fields are only in IDL, so if your future type is smart
enough to

Backwards compatibility is very similar to protobuf.

TBD: exact rules for backward compatibility

TBD: exact rules for forward compatibility

TBD: turning structure in algebraic type with compatibility


FAQ
===


Why Use Generic Types?
----------------------

Well, there are couple of reasons:

1. Different languages have different types, e.g. Python does have generic
   integer only, Java does not have unsigned integer types

2. Fixed width types are not good constaint anyway, valid values have often
   much smaller range than that of the type, so this is not a replacement for
   data validation anyway


Why No Default Values
---------------------

There are couple of reasons:

1. Default value is user-interface feature. And every service might want use
   it's own default value.

2. It's very application-specific if value that equals to default value may
   be omitted when serializing. And we want to use native structures for the
   language without any additional bookeeping of whether the value is default
   or just equals to it.


.. _Protobuf: https://github.com/google/protobuf
.. _CBOR: http://cbor.io/
