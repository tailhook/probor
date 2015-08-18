======
Probor
======


:Status: Proof of Concept

Probor is an extensible mechanism for serializing structured data on top of
CBOR_.

In additional to CBOR_ probor has the following:

1. A library to efficiently read data into language native structures

2. A schema definition language that serves as a documentation for
   interoperability between systems

3. A more compact protocol which omits field names for objects

4. Conventions to make schema backwards compatible


Why?
====

We like CBOR_ for the following:

1. It's IETF standard

2. It's self-descriptive

3. It's compact enought

4. It's extensive_ including mind-boggling_ things

5. It has implementations in most languages

.. _extensive: http://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml
.. _mind-boggling: https://github.com/paroga/cbor-js/issues/3

What we lack in CBOR_:

1. No schema definition, i.e. can't check interoperability between systems

2. Transmitting/storing lots of objects is expensive because keys are encoded
   for every object

3. No standard way to cast "object" (i.e. a map or a dict) into native typed
   object


Comparison
==========

This section roughly compares similar projects to see second our arguments in
"Why?" section. Individual arguments may not be very convincing by they are
reasonable enough in total.


Probor vs Protobuf
------------------

Protobuf_ Can't parse data if no schema known. *Probor* is not always totally
readable, but at least you can unpack the data using generic *cbor* decoder and
look at raw values (presumably without key names).

And it's not only hard when schema is unknown, but when you have a schema but
no code generated to inspect it. For example if you have a java application,
but want to inspect some code in python. You need a pythonic code generator and
generate code before you can read anything with *protobuf*.

*Probor* also has debugging (non-compact) mode in which it may encode object
and enums by name so you can easily understand the values. You can also keep
key names for most objects except ones that are transmitted in large
quantities, because compact and non-compact formats are compatible. You are in
control.

The types that *Protobuf* generates are not native. So they are larger and
hard to work with. Because code is generated you usually can't add methods
to the object itself without subtle hacks. *Probor* tries to provide thin layer
around native objects.

Also working with a code generation is inconvenient. *Protobuf* has a code
generator written in C++ which you need to have installed. Moreover you often
need another version of protobuf code generator for every language. *Probor*
works without code generation for all currently supported languages
by providing simple to use macros and/or annotations to native types. We may
provide code generation facilities too for bootstrapping the code, but they
should be done purely in the language they generate.

On the upside of *Protobuf* it can deserialize lookup object and serialize
again without loosing any information (even fields that are not in his version
of a protocol). For *probor* it's not implemented in current libraries for
effiency reasons, but it can be done with apropriate libraries anyway.

.. _Protobuf: https://github.com/google/protobuf

Probor vs Avro
--------------

Avro_ needs a schema to be transported "in-band", i.e. as a prefix to a data
send. We find this redundant.

Also *Avro* types are somewhat historic from C era. We wanted modern algebraic
types like they are in Rust or Haskell.

Also *avro* file format is not in IETF spec and does not have such interesting
extensions like CBOR_ `has`__.

__ mind-boggling_

.. _avro: https://avro.apache.org/


Probor vs Thrift
----------------

Thhift doesn't have good description of the binary format (in fact it has two
both are not documented in any sensible way) unlike CBOR_ which is IETF
standard. Do the data is hard to read without having code generated in advance.

*Thrift* also has ugly union type from 1990x, comparing to nice algebraic types
which we want to use in 2015.

*Thrift* relies on code generation for parsing data which we don't like because
it makes programs hard to build and it's hard to integrate with native
types (i.e. add a method to generated type).

Also *thrift* bindings usually have some implementation of *services*
which usually is a cruft because there are too much ways for dealing with
network in each language to have all of them implemented by thrift authors.
Furthermore *thrift* has long gistory of generating code that can't be network
IO agnostic.

.. _thrift: http://thrift.apache.org/


Probor vs Capnproto
===================

*Capnproto* has ugly and complex serialization format which is useful for
mapping values directly into memory without decoding. But its more complex to
implement correctly than what we target for. We also wanted compact encoding
which *Capnproto* has but it's built on top of already hard to understand
encoding and complicates things even more.

*Capnproto* like other relies on code generation with ugly protocol objects
as result of decoding, but we wanted native types.

.. _capnproto: https://capnproto.org/


Look-a-Like
===========

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
  IDL's (like protobuf, thrift or capnproto)

The structure serialized with probor will look like (diplaying json for
clarity, in fact you will see exact this data if decode CBOR and encode with
JSON):

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
   field instead of using ``Option<T>``
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


.. _CBOR: http://cbor.io/
