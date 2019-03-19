# Week 3: File Authentication System with SHA256

The naive approach to authenticate a very large video file hosted on a web server is for the browser to download the entire file and check that its hash is equal to the authentic hash value provided by the website via some authenticated channel. Unfortunately, this means that the video can only be played after the *entire* file has been downloaded.

Our goal in this project is to build a file authentication system that lets browsers authenticate and play video chunks as they are downloaded without having to wait for the entire file. The website generates the authentic hash value as follows:

1. Break down file into 1KB blocks (1024 bytes).
1. Compute hash of the last block and append value to the second to last block.
1. Compute hash of this augmented second to last block and append value to the third from the end.
1. Repeat process to the first block.

```
    |<- SHA256 <-|    |<--- SHA256 <-|    |<- SHA256 <-|
    |            |    |              |    |            |
    |    ,-------+----|--.   ,-------+----|--.   ,-----+------.
    h0   | block #0 | h1 |   | block #1 | h2 |   |  block #2  |
         `---------------'   `---------------'   `------------'
```

The final hash value `h0` (hash of the first block with its appended hash) is distributed to users via the authenticated channel.

Browser downloads the file one block at a time, where each block includes the appended hash value as above.

1. When the first block is received the browser checks that its hash is equal to `h0`.
1. When the second block is received the browser checks that its hash is equal to `h1`.
1. The process continues until the last block.

Each block is authenticated and played as it is received and there's no need to wait until the entire file is downloaded.

We use an existing crypto library, [sha2], for its SHA256 function.

[sha2]: https://crates.io/crates/sha2

If the file size is not a multiple of 1KB then the very last block will be shorter than 1KB, but all other blocks will be exactly 1KB.

Compute the hash `h0` of a given file F and verify blocks of F as they are received by the client.

## Usage

```
Usage: ./target/debug/w3-file_auth INPUT_FILE OUTPUT_FILE [options]

Options:
    -v, --verify HASH   verify signed input file and output original file
    -h, --help          print this help menu
```

Sign the video file:

```
$ ./target/debug/w3-file_auth /path/to/video.mp4 /path/to/video.mp4.signed
Hash 0: 03c08f4ee0b576fe319338139c045c89c3e8e9409633bea29442e21425006ea8
File created: /path/to/video.mp4.signed
```

Verify the signed video file, and output the video file:

```
$ ./target/debug/w3-file_auth /path/to/video.mp4.signed /path/to/video.verified.mp4 -v 03c08f4ee0b576fe319338139c045c89c3e8e9409633bea29442e21425006ea8
Verified: true
File created: /path/to/video.verified.mp4
```
