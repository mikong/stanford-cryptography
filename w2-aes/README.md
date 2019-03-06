# Week 2: AES in CBC and CTR modes

In this project, we have two encryption/decryption systems:

* AES in CBC mode
* AES in counter mode (CTR)

In both cases the 16-byte encryption IV is chosen at random and is prepended to the ciphertext.

For CBC encryption, we use the [PKCS7 padding scheme][PKCS7].

[PKCS7]: https://en.wikipedia.org/wiki/Padding_(cryptography)#PKCS#5_and_PKCS#7

In this assignment, we are given an AES key and a ciphertext (both are hex encoded) and our goal is to recover the plaintext.

We use an existing crypto library, [aes-soft], for its built-in AES function, and we only implement the CBC and CTR modes.

[aes-soft]: https://crates.io/crates/aes-soft

#### Problem 1

* CBC key: `140b41b22a29beb4061bda66b6747e14`
* CBC Ciphertext 1:

```
4ca00ff4c898d61e1edbf1800618fb2828a226d160dad07883d04e008a7897ee2e4b7465d5290d0c0e6c6822236e1daafb94ffe0c5da05d9476be028ad7c1d81
```

#### Problem 2

* CBC key: `140b41b22a29beb4061bda66b6747e14`
* CBC Ciphertext 2:

```
5b68629feb8606f9a6667670b75b38a5b4832d0f26e1ab7da33249de7d4afc48e713ac646ace36e872ad5fb8a512428a6e21364b0c374df45503473c5242a253
```

#### Problem 3

* CTR key: `36f18357be4dbd77f050515c73fcf9f2`
* CTR Ciphertext 1:

```
69dda8455c7dd4254bf353b773304eec0ec7702330098ce7f7520d1cbbb20fc388d1b0adb5054dbd7370849dbf0b88d393f252e764f1f5f7ad97ef79d59ce29f5f51eeca32eabedd9afa9329
```

#### Problem 4

* CTR key: `36f18357be4dbd77f050515c73fcf9f2`
* CTR Ciphertext 2:

```
770b80259ec33beb2561358a9f2dc617e46218c0a53cbeca695ae45faa8952aa0e311bde9d4e01726d3184c34451
```
