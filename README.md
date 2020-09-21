# HPermission

![](https://ftp.bmp.ovh/imgs/2020/09/3bff8a66d6100f4a.png)

A high performance authentication gateway

## Why we need this?

In an ordinary micro-service based application architecture, 
each service should take charge of its own business, they should not 
has the knowledge of "who can visit me?", so a authentication gateway 
is necessary.

Currently, there are several available authentication gateways,
but these are somehow too heavy and too hard to use.

So we decided to develop this, a high performance authentication gateway,
which is also lightweight and easy to use.

## High performance

As the name of this project implies, this is a H(igh) P(erformance) Permission checking program.

It's important to make an authentication gateway to be fast, because each request has to pass it.

To archive this:
- We use Rust, since it's an relative safe and low level language, while still has all
  necessary properties of a modern language.
- We use actix, since it's the fastest web framework in Rust.
- We'll try all we can do to optimize this project.

And another thing need to be mentioned,
the program should be stateless,
so it can be scaled out easily.

## How to use

See our [wiki](https://github.com/shuosc/HPermission/wiki).

## How to contribute

See our [CONTRIBUTING.md](./CONTRIBUTING.md)
