# tinydns

## Current Status

This Project is still a WIP. As I currently have a pretty busy schedule updates
might be sporadic.

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
- [ ] Add Web Interface

Find more TODOs by running the following in the project directory:
```sh
grep --include="*.rs" -rni "todo"
```
