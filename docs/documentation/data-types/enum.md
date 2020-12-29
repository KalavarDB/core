---
has_children: false
layout: default
title: Enum
parent: Data Types
grand_parent: Documentation
nav_order: 22
---

# Enum
Allows you to specify values, must link to an `EnumTable` (documentation coming)

### Information
- Limits: Must be linked to an `EnumTable` to derive allowed values
- Encoding: Binary

### Explanation
An enum must be linked to an undocument `EnumTable` in order to derive the values it is to allow, if no link is found an error will be thrown.

#### Example EnumTable
For this example we will define some permissions for an reference forum

|Name|Value|
|:---:|:---:|
|CanVote|0|
|CanComment|1|
|CanPost|2|
|Admin|3|

The above table, means that a user can have either the `CanVote`, `CanComment`, `CanPost` or `Admin` permission, (in our case each of these permissions implies the prior ones, so this is fine)