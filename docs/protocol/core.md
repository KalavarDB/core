> ## ⚠️ Warning ⚠️
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

## Domains
Kalavar will bind itself by default to port `1234` of `localhost`, should the client attempt a connection to any other port, it might be worth attempting a connection here as well. Kalavar will listen for incoming data on any port specified by the user, Or, if enabled in config, the current IP address of the system it is running on

An example connection URL would just be `localhost:1234`

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


~~Previous~~

[Next](auth.md)