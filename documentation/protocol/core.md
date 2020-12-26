> ## :warning: Warning :warning:
> #### The documentation you are reading is neither implemented, nor finalised. please do not begin an inplementation of the protocol until this message has been removed. Thank you.

# Core Documentation
This page represents some core information about the protocol. It is strongly recommended that you read this page in its entirety before implementing the Kalavar protocol


# Contents

- [Connection](#connection)
    - [Domains](#domains)

- [Payload](#payload)
    - [Payload Structure](#payload-structure)
    - [Payload Order](#payload-order)
    

    
# Connection
Kalavar uses TCP to transfer data to and from the client. The 


# Payload
Each transmission of data between the client and the server, is referred to as a "payload". This term is used throughout the documentation so please keep it in mind.

## Payload Structure
All payloads have a specific structure, that they must follow for the payload to be processed. That structure can be seen below:
```
<Operation Code> <MD5 Checksum>
<body>
<empty line>
```

An example of a fully qualified `QUERY` payload, is below:
```
0 137E450BD15D6EBCE2E1CA459573B7FA
GET my_database.A
FIELDS: "name", "email", "pass"
I-JOIN my_database.B
FIELDS: "banned", "two_factor", "admin"

```

The above payload consists of an `Operation Code` or `OpCode`, followed by an MD5 hash of the payload `body` simple `GET` query as the payload `body`.

In order to verify that the payload has not been tampered with during transmission, the server will parse the `OpCode`, and then verify the `checksum`, if the `checksum` matches the body, the payload will be accepted as genuine and processed according to it's `OpCode` automatically.

If a payload is modified in transmission, the client will be notified, by receiving the following payload:
```
20 7EF573B4492E3FA035605BAF1FFD645A
Payload Invalid
```
If the client receives such a payload from the server, they should immediately drop the payload they sent, and report a "Tamper" error to the end user, to let them know someone may be snooping in on their queries.


~~Previous~~

[Next](auth.md)