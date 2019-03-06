# Programming Assignments for Cryptography I by Stanford University

The programming assignments for Coursera Cryptography I by Stanford University are optional, and are only now being worked on (WIP) after I have finished the course. I'll be using [Rust](https://www.rust-lang.org/) to work on these.

### Week 1: [Many Time Pad][week-1]

Let us see what goes wrong when a stream cipher key is used more than once. Given ciphertexts encrypted with the same stream cipher key, our goal is to decrypt the target ciphertext to get the secret message.

### Week 2: [AES in CBC and CTR modes][week-2]

Implement two encryption/decryption systems, one using AES in CBC mode and another using AES in counter mode (CTR). Given pairs of AES key and ciphertext, our goal is to recover the plaintext.

### Week 3: [File Authentication System with SHA256][week-3]

Our goal in this project is to build a file authentication system that lets browsers authenticate and play video chunks as they are downloaded without having to wait for the entire file.

### Week 4: [Padding Oracle Attack][week-4]

Let's experiment with a padding oracle attack against a toy website. Knowing that the website is vulnerable to a CBC padding oracle attack, our goal is to decrypt the ciphertext in the sample intercepted URL.

[week-1]: w1-many_time_pad/
[week-2]: w2-aes/
[week-3]: w3-file_auth/
[week-4]: w4-padding_oracle_attack/
