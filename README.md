# tinydns

## Goal

The goal of this application is to serve as a hybrid DNS server (i.e. implementing both, resolver and name server capabilities).
Admins should be capable of configuring additional dns records and/or blocking others.

Furthermore, the service should come equipped with a user friendly web interface, easing its configuration as well as enabling
evaluation of query metrics.

## Motivation

The main motivation for tinydns is the capability to block troublesome / malicious websites in home networks.

### To-Do
- [ ] Add truncation support for large datagrams
- [ ] Add Message Compression

Find more TODOs by running
```sh
grep --include="*.rs" -rn "TODO"
```