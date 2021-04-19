# keycrypt-recrypted
I am bored this Sunday and I want to make keycrypt faster than the original Rust version I wrote 3 years ago.

## Go version

Can encrypt the entire KJV bible .txt within half a second (on my slow CPU). Can decrypt the encrypted bible in 2 and a half seconds.
That's 340,435 words decrypted a second. Only 137 SLOC (excluding test files). Not running in parallel. I considered it, but I think
it is unnecessary since it is already so fast.

Below are the profiles for encrypting and decrypting the King James Version bible. One notable fact the decryption profile shows, is
how the (compiled) regular expression that extracts encrypted word boundaries *before* decrypting words is actually what takes 65%
of the execution time. If I had made the word decrypting parallel, the maximimum percentage gain I could have saved would be only 4%.
Always benchmark!

![Encrypt profile](go/encrypt-profile.svg)
![Decrypt profile](go/decrypt-profile.svg)
