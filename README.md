# pa02 Crack Diffie Helmen
Your job is to program Eve the eavesdropper in Rust!
![](eve.jpg)

## Part 1: Crack the baby version of DH
Re-read:
https://www.cnsr.dev/index_files/Classes/Security/Content/07-CryptoMath.html

Crack a weaker conceptual cousin to DH, using secret exponents without mod.
This weaker version is described here:

1. Alice and bob email each other to pick a shared base:
$`SharedBase`$

2. Alice emails the shared base to the power of her secret:
$`AliceBroadcasts = SharedBase^{AliceSec}`$

3. Bob emails the shared base to the power of his secret:
$`BobBroadcasts = SharedBase^{BobSec}`$

4. Alice generates the shared secret:
$`BobBroadcasts^{AliceSec}`$

5. Bob generates same shared secret:
$`AliceBroadcasts^{BobSec}`$

Eve works at google on the gmail team.
Eve is curious what Alice and Bob need that secret for.
Your goal is to write a simple Rust program to crack the secret, to satisfy Eve's curiosity.

### unit\_tests/
Trace this in `gdb` to check it out!
We randomize some small baby eve problem instances.

### stdio\_tests/
Your program will assume that three numbers are entered via the keyboard input, 1 at a time, Alice's Broadcast, Bob's Broadcast, and the Public Base.
It should then print Alice's Secret, Bob's Secret, and the Mutual Secret on one line.

### arg\_tests/
You will be given 2 command line arguments containing the input file and the output file.
From the input file, you will read Alice's Broadcast, Bob's Broadcast, and the Public Base from the input file.
In the output file, put Alice's Secret, Bob's Secret, and the Mutual Secret.

### Running your Code
All code should fit in the `src/` directory (we have your files already there).
Your main driver and functions will be in `src/main.rs`.
More information about Cargo and how it is is in section 1.3 in:
['The Book'](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html).
Below are references to the commands you will use the most.

- `cargo run` : Compiles and runs your code, allowing interaction with it for stdio testing.
    - May require a --bin if you have multiple binaries (as this assignment does with it's unit test).
- `cargo run arg1 arg2 arg3` : Compiles and runs your code, passing along any number of arguments you need, for arg-based testing.
- `cargo build` : Compiles your code and puts it in `target/debug/name_specified_in_cargo.toml`

### Hint
The number types in Rust only go up to 64-bit (128-bit if you are using u128).
This is not large enough to allow for large keys to be cracked.
All inputs will be small enough to be handled in the u64/f64 variable types.
The usage of a BigInt is not required for the basic baby eve.

## Part 2: Big Eve - Crack the real DH (optional bonus)
Re-read:
https://www.cnsr.dev/index_files/Classes/Security/Content/08-AsymmetricEncryption.html

If you complete this, you can replace a grade on any past or future paNN!

Deliverables:
* A solution that mimics the baby version in structure but with the real logic (brute force from the bottom up).
* A correct randomized unit test, which creates real DH problems using very small numbers, and tests for their crack.

To complete this section, you should write:
* the `big_eve` function in `src/eve.rs`, and
* a second unit test as `unit_tests/randomized_test_big.rs` that mimics the existing baby one.

To avoid triggering `grade.sh` security features, run `.admin_files/setup.sh` before testing in `grade.sh` itself.

