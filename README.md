# Phi base Random Encoder Aimed for Cryptography Hardening

## Overview

`preach` is a Rust program using the properties of the Phi base to harden cryptography.
Cryptography is base on the fact that a payload crypted by a symmetric key give always same result.

`preach` allow multiple results with same payload and key, but still allowing recipient to decrypt.

## How it works ?

`preach` encode each byte (0-256) on three bytes on a random choice between all forms of integer coded in Phi base.

## Why ?

Attacker can try brut force it he know that a payload is always starting the same, or contain always
same data at a given position.
With `preach`, attacker cannot know if the payload changed, or key changed, or both.
Some bytes can be encoded up to 233 different way on three bytes.
A same payload will almost never be twice again and if it happen the symmetric key would have been changed already.

## How to use it ?

`preach` have to be used on payload *before* crypting. As size of payload becomes three time more,
compressing is recommanded *after* crypting in order not to help attacker with compression headers.







