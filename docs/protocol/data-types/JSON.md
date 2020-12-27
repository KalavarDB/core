---
has_children: false
layout: default
title: JSON
parent: Data Types
grand_parent: Protocol
nav_order: 2
---
# JSON
JSON values are groups of key-value combinations, they have no upper maximum length and are always encoded using UTF-8.

### How do i use a `json` value?
JSON values are formatted by wrapping their content content within the `'` (single quote) character in your queries. This signifies a JSON body, and any newlines or query content within the section is ignored.

### Information
- Max Length: infinite\*
- Encoding: UTF-8
- Category: Text

### Errors
N/A


*A string may be infinite length, but the true length of the string is dependant on the limitations of the hardware the system is running on