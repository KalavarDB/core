---
has_children: false
layout: default
title: Payload Breakdown
parent: Protocol
grand_parent: Documentation
nav_order: 3
---

# Payload
Each transmission of data between the client and the server, is referred to as a "payload". This term is used throughout the documentation so please keep it in mind.

## Payload Structure
All payloads have a specific structure, that they must follow for the payload to be processed. That structure can be seen below:
```
<Operation Code> <SHA-1 Checksum>
<body>
<empty line>
```

An example of a fully qualified `QUERY` payload, is below:
```
0 E97AD0AFA9C157357F9EFC6E3EE1D29687385FCB
GET my_database.A
FIELDS: "name", "email", "pass"
I-JOIN my_database.B
FIELDS: "banned", "two_factor", "admin"

```

The above payload consists of an `Operation Code` or `OpCode`, followed by an SHA-1 hash of the payload `body` simple `GET` query as the payload `body`.

In order to verify that the payload has not been tampered with during transmission, the server will parse the `OpCode`, and then verify the `checksum`, if the `checksum` matches the body, the payload will be accepted as genuine and processed according to it's `OpCode` automatically.

If a payload is modified in transmission, the client will be notified, by receiving the following payload:
```
20 9ACEE9D9B4EE5C370A7732B5118293744D0106F8
Payload Invalid
```
If the client receives such a payload from the server, they should immediately drop the payload they sent, and report a "Tamper" error to the end user, to let them know someone may be snooping in on their queries.