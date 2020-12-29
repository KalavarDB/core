---
has_children: false
layout: default
title: Booleans
parent: Data Types
grand_parent: Documentation
nav_order: 3
---

# Boolean
A binary value used to represent true (1) or false (0).


### Information
- Maximum Length: 1 word
- Encoding: Binary
- Accepted Values: `True`, `False`

### Dangers:
#### Typecasting:
Casting a boolean value to an integer can be dangerous, some languages treat 1 and 0 as true and false respectively within logic gates, and this can cause unexpected logic errors when you cast to an integer of any kind.