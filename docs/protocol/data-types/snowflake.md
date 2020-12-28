---
has_children: false
layout: default
title: Snowflake
parent: Data Types
grand_parent: Protocol
nav_order: 8
---

# Snowflake
A 64 bit unsigned integer, comprising multiple datapoints ensuring guaranteed uniqueness over a large number of active connections, even in our asynchronous environment.

### Information
- Bits: 64
- Encoding: Denary/Decimal
- Signature: None


### Breakdown

|Bit-range|Name|Description|
|:---:|:---:|:---:|
|0 - 12|ID|A 12 bit integer, incremented for every id generated on this connection|
|13 - 17|Connection Identifier|A 4 bit integer, identifying the process making the id|
|18 - 22|Internal Worker ID| The id of the service worker that issued the snowflake|
|23 - 64|Timestamp|The number of milliseconds from the (pre-defined*) Epoch|


### Asterisks
*The Epoch used to generate the snowflake, can be set in the column configuration, or during the creation of the snowflake itself, either by the client library, or by the server

