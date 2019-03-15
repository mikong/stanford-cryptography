# Week 4: Padding Oracle Attack

Suppose we, as an attacker, wish to steal secret information from our target website crypto-class.appspot.com. We found out that the website embeds encrypted customer data in URL parameters such as this:

```
http://crypto-class.appspot.com/po?er=f20bdba6ff29eed7b046d1df9fb7000058b1ffb4210a580f748b4ac714c001bd4a61044426fb515dad3f21f18aa577c0bdf302936266926ff37dbf7035d5eeb4
```

We intercept the URL listed above and guess that the ciphertext following the "po?er=" is a hex encoded AES CBC encryption with a random IV of some secret data about Alice's session.

We also discover that the website is vulnerable to a CBC padding oracle attack. When a decrypted CBC ciphertext ends in an invalid pad the web server returns a 403 error code (forbidden request). When the CBC padding is valid, but the message is malformed, the web server returns a 404 error code (URL not found).

Given the above, our goal is to decrypt the ciphertext listed above. We can send arbitrary HTTP requests to the website of the form:

```
http://crypto-class.appspot.com/po?er="our ciphertext here"
```

We should be able to decrypt the given ciphertext one byte at a time by observing the resulting error code. For each byte, we send up to 256 HTTP requests. Note that the first ciphertext block is the random IV. The decrypted message is ASCII encoded.
