# Hack Steps

1. Craft an HTML form for changing the email address with an auto-submit script and a meta tag that drops the Referer header from the request
2. Deliver the exploit to the victim
3. The victim's email will be changed after they trigger the exploit

# Run Script

1. Change the URL of the lab
2. Change the URL of the exploit server
3. Start script

```
~$ cargo run
```

# Expected Output

```
❯❯ Delivering the exploit to the victim.. OK
🗹 The victim's email will be changed after they trigger the exploit
🗹 Check your browser, it should be marked now as solved
```