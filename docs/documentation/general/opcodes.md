---
has_children: false
layout: default
title: Operation Codes
parent: Protocol
grand_parent: Documentation
nav_order: 1
---
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
|10|Connect|Server|Server|Sent internally to the server to inform it that a new connection has been received and accepted, and that it should provide details accordingly|
|11|Reserved|N/A|N/A|N/A|
|12|Reserved|N/A|N/A|N/A|
|13|Reserved|N/A|N/A|N/A|
|14|Reserved|N/A|N/A|N/A|
|15|Reserved|N/A|N/A|N/A|
|16|Reserved|N/A|N/A|N/A|
|17|Reserved|N/A|N/A|N/A|
|18|MEMUSE|Server|Server|Sent by the server to itself (internal comms) to provide the program with an idea of just how much ram is in use at a given time. Allows the system to selectively move data into and out of memory according to memory usage|
|19|Terminate|Either|Either|Used to signal that this connection will not be receiving on the opposite end, and should be closed immediately. Also sent if an invalid Opcode 8 payload is provided to the server|
|20|Payload Tamper|Either|Either|This opcode should only be sent, or received if there has been evidence of payload tampering. It will immediately be responded to with an Opcode 19 Terminate payload|