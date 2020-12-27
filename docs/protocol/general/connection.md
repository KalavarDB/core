---
has_children: false
layout: default
title: Connections
parent: General
grand_parent: Protocol
nav_order: 2
---
# Connection
Kalavar uses TCP to transfer data to and from the client. The 

## Domains
Kalavar will bind itself by default to port `1234` of `localhost`, should the client attempt a connection to any other port, it might be worth attempting a connection here as well. Kalavar will listen for incoming data on any port specified by the user, Or, if enabled in config, the current IP address of the system it is running on

An example connection URL would just be `localhost:1234`