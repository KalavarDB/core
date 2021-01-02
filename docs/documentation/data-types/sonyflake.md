---
has_children: false
layout: default
title: SonyFlake
parent: Data Types
grand_parent: Documentation
nav_order: 8
---

# SonyFlake
A 63(?) bit unique identifier, based off the [Snowflake](snowflake.md) spec. For detailed implemtnations, see Sony's official open source repository [here](https://github.com/sony/sonyflake).

### Information
- Bits: 63
- Encoding: Denary/Decimal
- Signature: None

### Breakdown

|Bit-range|Name|Description|
|:---:|:---:|:---:|
|0 - 16|Machine ID|A 16 bit integer, unique to the instance which generated this id|
|16 - 24|Sequence|An 8 bit integer, incremented for every id that this host generates, 8 bits means 255 ids per machine ID, Kalavar increments both automatically to prevent collisions|
|24 - 63|Timestamp|The number of 10 millisecond intervals since the (pre-defined*) Epoch|


### Asterisks
*The Epoch used to generate the snowflake, can be set in the column configuration, or during the creation of the snowflake itself, either by the client library, or by the server

