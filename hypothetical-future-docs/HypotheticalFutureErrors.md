## Records cannot have `[ deprecated ]` required fields

Records have a lot in common with _field lists_, but they aren't quite the same!
Consider this `args:` field list:

```
post sendReply
    args:
        * userId : Int
        * title : String [ deprecated ]
```

Here, `title` is a required deprecated field, which means the sender must
provide a `title`, but the receiver will consider `title` optional.

To represent this in type-checked languages, WireFunc compiles `title` to
different types for sender and receiver.

* The sender will get a `sendReply` function where `title` is a `String`.
* The receiver will get a function where `title` is instead a `Maybe String`.

This works just as well for field lists like `args:` as it does field lists
like `response:`. With field lists, there is no problem!

### The Problem

This approach doesn't work for records, because records have to compile to one
consistent type everywhere. One reason this is necessary is that records can be
nested within other types.

For example, this `EmailReply` record could be nested within a custom type:

```
record EmailReply
    * userId : Int
    * title : String [ deprecated ]


type Reply
    | Email EmailReply
    | TextMessage TextReply
    | CarrierPigeon PigeonReply
```

Now the shape of `Reply` depends the shape of `EmailReply`. However, the whole
point of deprecated required fields is that have a different shape depending on
whether they are being sent or received!

Should `Reply` use the version of `EmailReply` where `title` is a `String`, or
the one where it is a `Maybe String`? It has to be one or the other, but
favoring the sender's type messes up the receiver, and vice versa.

### Reliable Solution: `title : String`

A reliable alternative to making `title` required and deprecated is to make
it an optional field with a fallback:

```
record EmailReply
    * userId : Int
    ~ title : String [ deprecated ] [ fallback ]
```

Now `title` is a `String` everywhere, which means:

* Senders always know to send it.
* Receivers always know to specify a fallback `String`.
* It has one consistent type, so it can be nested in other types.

This solves the problem without breaking backwards compatibility!

### Risky Solution: `title : Maybe String`

Sometimes you may find that a fallback value is not what you want. If that
happens, there is a riskier alternative you can consider: a deprecated optional
field that has no `[ fallback ]`.

```
record EmailReply
    * userId : Int
    ~ title : String [ deprecated ]
```

Here, since `title` is optional and has no `[ fallback ]`, it will be a
`Maybe String` everywhere. That fixes the consistency problem, but introduces
another.

Since the sender now treats `title` as a `Maybe String`, our backwards
compatibility will depend on programmer discipline. We must be very careful to
have all senders always provide a `title`, even though its type permits
leaving it off!

This makes the previous "optional deprecated field with a fallback" approach the
most reliable alternative to "required deprecated field."
