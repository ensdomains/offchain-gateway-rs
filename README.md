# Offchain Gateway (Rust)

> [!WARNING]
> This repository is under construction 🚧. We are actively improving it hackathon-style.

This is a Rust implementation of the CCIP gateway. It allows you to issue unlimited gasless subdomains for your name, as well as to create, manage, and moderate namespaces.

> [!NOTE]
> This gateway is built to be an **Opinionated ENS Subname Issuer**, if youre looking for something more generic, please checkout [ccip-gateway], and [].

## Features

- CCIP Spec Compliant Gateway Endpoint
- `postgres` - Transparent Database backend
- `selfservice` - Enabled self-service for updating records

## Setup

### Run the gateway
The gateway is just a docker container / standalone binary. You can run it with docker-compose or just run the binary.

todo: running the gateway

### Deploy a Resolver
To be able to use your newly deployed gateway for your name is able to respond to incomming queries, you need to deploy a resolver. Luckily we have made this process super easy.

todo: resolver deployment explanation

### Set your Resolver

todo: set your resolver explanation

## Fork this 🍴
Don't like the default web-api endpoints? Want to add your own custom logic? Fork this repo and make it your own!

Any contributions back to the original repo are also greatly appreciated!

