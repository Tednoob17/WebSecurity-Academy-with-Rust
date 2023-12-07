## Hack Steps

1. Read password list
2. Brute force carlos password via password change functionality and change his password (login as wiener before every try to bypass blocking)
3. Wait 1 minute to bypass blocking
4. Login as carlos with the new password

## Run Script

1. Change the URL of the lab
2. Make sure the passwords file exists in the root directory (Authentication directory) or change its path accordingly
3. Start script

```
~$ cargo run
```

## Expected Output

```
⦗1⦘ Reading password list.. OK
⦗2⦘ Brute forcing carlos password.. 
❯❯ Elapsed: 1  seconds || Trying (16/101): zxcvbnm                                           
🗹 Correct password: biteme
🗹 Password was changed to: Hacked
❯❯ Elapsed: 2  seconds || Trying (23/101): asdfgh                                            
⦗3⦘ Waiting 1 minute to bypass blocking.. OK
⦗4⦘ Logging in as carlos with the new password.. OK
🗹  Finished in: 62 seconds
🗹 The lab should be marked now as solved
```

# Test Samples

This test is done using only 100 passwods. What about 10K passwords?
Or what about 100K passwords?

You can see the comparison I made with these numbers when solving the [Lab: Username enumeration via different responses](https://github.com/elqal3awii/WebSecurity-Academy-with-Rust/tree/main/Authentication/Username%20enumeration%20via%20different%20responses) to see the big difference in speed between Rust and Python and also between single-threaded and multi-threaded approaches in Rust.
