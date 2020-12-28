---
has_children: false
layout: default
title: CMYK
parent: Data Types
grand_parent: Protocol
nav_order: 12
---

# CMYK
A color structure used to refer to the respective `cyan`, `magenta`, `yellow`, and `black` values of a color.

It is stored as a tuple containing four (signed) 8bit integers, each will self-cap at 100% (100)

### Information

- Maximum Value: (100, 100, 100, 100) (Black)
- Allowed Types: Signed Integer(8)