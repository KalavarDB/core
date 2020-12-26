---
has_children: false
layout: default
title: General
nav_order: 1
---
# General
This page represents some core information about the protocol. It is strongly recommended that you read this page in its entirety before implementing the Kalavar protocol

> ## ⚠️ Warning ⚠️
> #### The documentation you are reading is neither implemented, nor finalised. please do not begin an inplementation of the protocol until this message has been removed. Thank you.


# Contents

- [Connection](#connection)
    - [Domains](#domains)

- [Operation Codes](#opcodes)

- [Payload](#payload)
    - [Payload Structure](#payload-structure)
    - [Payload Order](#payload-order)
    

    
# Connection
Kalavar uses TCP to transfer data to and from the client. The 

## Domains
Kalavar will bind itself by default to port `1234` of `localhost`, should the client attempt a connection to any other port, it might be worth attempting a connection here as well. Kalavar will listen for incoming data on any port specified by the user, Or, if enabled in config, the current IP address of the system it is running on

An example connection URL would just be `localhost:1234`

# Opcodes
> Operation codes (opcodes) are the integers that are always the first component of the payload. These range from 0 to 20 currently and a breakdown can be found below

| Opcode | Name |Source|Destination| Description |
|:---:|:---:|:---:|:---:|:---|
|0|Normal Payload|Either|Either|Any normal payload transmitted from the client to the server, or vice versa. Normally these are general `Query` payloads|
|1|Ping|Server|Client|Used to check for stale or dead connections and close them. Sent once every 42500 milliseconds|
|2|Pong|Client|Server|Sent by the client, upon the request of a Ping by the server.|
|3|Disconnect|Server|Client|A request to the client to close the connection, ~~see here~~ |
|4|Reconnect|Server|Client|A request to the client to re-initiate the connection. Usually sent in order to re-negotiate an encryption key|
|5|Shutdown|Client|Server|A request to the server to close the connection. Requests will be answered by an Opcode 3 Disconnect|
|6|Status|Client|Server|A request to the server to provide state information to the client. Only trusted clients are accepted. ~~see what this means~~|
|7|Hello|Server|Client|A payload defining the key and iv to use to encrypt the user credentials of the Opcode 8 payload|
|8|Authenticate|Client|Server|A payload defining the username and password of the user attempting to log into the database. should be encrypted using the information provided through Opcode 7|
|9|Ready|Server|Client|Information pertaining to the logged in user. Sent upon the accepting of a valid Opcode 8 payload|
|10|Reserved|N/A|N/A|N/A|
|11|Reserved|N/A|N/A|N/A|
|12|Reserved|N/A|N/A|N/A|
|13|Reserved|N/A|N/A|N/A|
|14|Reserved|N/A|N/A|N/A|
|15|Reserved|N/A|N/A|N/A|
|16|Reserved|N/A|N/A|N/A|
|17|Reserved|N/A|N/A|N/A|
|18|Reserved|N/A|N/A|N/A|
|19|Terminate|Either|Either|Used to signal that this connection will not be receiving on the opposite end, and should be closed immediately. Also sent if an invalid Opcode 8 payload is provided to the server|
|20|Payload Tamper|Either|Either|This opcode should only be sent, or received if there has been evidence of payload tampering. It will immediately be responded to with an Opcode 19 Terminate payload|

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