## Hack Steps

1. Obtain a valid session
2. Fetch the login2 page
3. Start brute forcing the mfa-code of carlos
4. Fetch carlos profile

## Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

## Expected Output

```
⦗1⦘ Obtaining a valid session.. OK
⦗2⦘ Fetching the login2 page.. OK
⦗3⦘ Start brute forcing the mfa-code of carlos..
❯❯ Elapsed: 2 minutes || Trying (1400/10000) 1124 => Wrong
❯❯ Elapsed: 2 minutes || Trying (1467/10000) 0164 => Wrong
🗹 Correct Code: 1468
⦗4⦘ Fetching carlos profile.. OK
🗹 Finished in: 3 minutes
🗹 The lab should be marked now as solved
```
