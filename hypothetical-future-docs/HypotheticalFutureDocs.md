> ## WireFunc Hypothetical Future Docs
>
> WireFunc does not exist yet! I'm writing the docs early because:
>
> 1. I find that writing documentation often reveals design problems.
> 2. I want to share the design with people to get feedback.
>
> If you have feedback, please [ping me on Twitter](https://twitter.com/rtfeldman)!

# Introduction

Sometimes we do a production deploy and things break for some of our end users.
This happens despite the code having passed all of our automated tests, as well
as manual QA on the parts of the site where the code changed.

Common culprits include changes to how our app talks to our servers. For example:

1. We rename a JSON field on a particular page, on both client and server. However, it turns out another page was still hitting the same endpoint using the old name. That field then appeared to be "missing" to the other page, breaking it.
2. We add a new endpoint, change *all* of our client code to use it, and retire the old one. This works for everyone who visits the site after the deployment, but some users still had a tab open right when we deployed it. That page stopped working for them becaue it was trying to hit the retired endpoint, and they got frustrating error messages until they gave up and hit Refresh - at which point they got the new client code and everything started working again.

We don't want to break things for our users. How can we prevent these client-server miscommunications from slipping through our deploy process?

## Policies

Some organizations adopt policies to prevent these situations. The most
common ones are:

### Never Change Things

If you never rename or remove JSON fields, everything will always be backwards compatible.

This prevents the problem but creates other problems. Code and JSON payloads get increasingly bloated, and the only way to remove the bloat is to make new endpoints, mark old ones as deprecated, and leave them around for awhile but stop using them. This makes minor cleanups costly, discouraging them in favor of waiting until things get really bad to do a full endpoint migration. This normalizes having client-server code be in a perpetual state of being somewhat unpleasantly messy.

### Never Require Things

If all clients assume that any field could be missing at any time, and code defensively around this possibility, then if things get renamed or removed, clients will recover as gracefully as possible.

This is what Google's Protocol Buffers do. They embrace this policy so strongly that they [removed the concept of required fields](https://github.com/protocolbuffers/protobuf/issues/2497#issuecomment-267422550) in version 3.

This creates much more serious problems than "never change things." If all fields are optional, this means application developers must handle each field in one of three ways:

1. Choose a default. This may work sometimes - e.g. "if the `notifications` field is missing, default to an empty array." Other times there may not be a reasonable default - e.g. "if the `userId` field is missing, default to `0`." A bad default for a field like `userId` can cause much nastier problems for the end user than than their seeing an error message.
2. Check for presence and fail fast. In other words, "if we get the message and `userId` is missing, immediately fail with an error message." If a field being missing results in an immediate failure, that's the same thing as what happens if a field is "required." So "instead of requiring it, have it be optional but fail fast" is not a meaningful change in policy.
3. Check for presence later. In other words, have defensive checks all over the code base. Perhaps you'd have a linter rule that said "every time you write `foo.bar` you must wrap it in a conditional which verifies that `foo` is not `null`." Without such checks, you'd potentially see bugs that are distant in the code base from the root cause that a field was missing from JSON.

## Tools

These policies have a cost to developer experience, and potentially to user experience as well. They all discourage removing or renaming things, which are fundamental techniques for keeping a code base clean. Some policies increase payload size over time, negatively impacting application performance. Others make application code more error-prone, resulting in more bugs for end users.

Tools can help, though! For example, Apollo Engine's [Schema History](https://www.apollographql.com/docs/engine/features/schema-history.html) feature for GraphQL informs you when you're about to make a backwards-incompatible change, so you can decide whether that's actually what you want to do. It can also tell you how often certain fields or endpoints are being used in the wild, so if you want to remove something deprecated, you can be informed about the impact that might have.

## WireFunc

WireFunc is a tool with ambitious goals. It aims to:

1. **Improve** developer experience. Even if you had no end users to consider, WireFunc should make your life easier.
2. **Reduce** payload size, so the same application performs better than before it used WireFunc.
3. **Prevent** deploys from breaking production due to unexpected client-server miscommunications.

To learn about how WireFunc works, read on!

# Chapter 1. Single Source of Truth

Suppose we have a JavaScript client and a Node.js server running Express.

Here's how we might write some code to have them communicate:




## Payload Minification

Since WireFunc is generating the code on sides of the connection, it can use
a more compact format in production than in development.

In development builds, WireFunc prioritizes making things easy to read on the
wire, so you get a nice experience when looking at them in developer tools. By
default it uses JSON as the wire format, and also adds some extra information,
like showing timestamps both in UTC as well as in your local time zone.

In production builds, WireFunc uses a compact binary format when talking between
servers, and minified JSON when talking to the browser.

> Browsers have a performance penalty for converting binary data into strings,
> which they do not have for parsing JSON. This is why JSON is the default
> format choice when communicating with browsers. However, if you want to, you
> can use the compact binary format in the browser as well!
>
> The upside will be that your payloads will take up less space on the wire.
> The downsides will be that you'll need a polyfill for older browsers, your
> compiled code bundle size will get bigger, and your application will be a bit
> slower to decode the payload.

The minified JSON format works basically the same way as minified JavaScript:
although your schema specifies that the field should be called `username`, on
the wire it might be shortened to `a`. WireFunc will generate code to translate
between the two, like so:

```javascript
user.username = json.a;
```

Minifiers like UglifyJS will shrink this renaming code to something so short it
practically disappears:

```javascript
u.u=j.a;
```

This tiny and highly cacheable code snippet saves bytes and parse time on every
single request that uses this field!

> It's conceivable that a Babel plugin could make this *literally* disappear by
> coordinating the minifiers - such that `user.u` became `user.a` everywhere
> in the minified code base, and then any lines like `user.a = json.a` could be
> replaced by directly parsing the JSON into `user` - in other words,
> `user = JSON.parse(str)` - making the wire minification code disappear.

# Chapter 2. Migration Assistant

# Chapter 3. GraphQL Support

# Documentation

## Syntax

## Migrations

## Tags

## Tags

## FAQ

