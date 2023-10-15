# Hack Steps

1. Inject payload into the XML-based check stock request to issue a DNS lookup to burp collaborator using a parameter entity
2. Check your burp collaborator for the DNS lookup

# Run Script

1. Change the URL of the lab
2. Change the domain of the burp collaborator
3. Start script

```
~$ cargo run
```

# Expected Output

```
⟪#⟫ Injection point: XML-based check stock request
❯ Injecting payload to issue a DNS lookup to burp collaborator using a parameter entity.. OK
🗹 Check your burp collaborator for the DNS lookup
🗹 Check your browser, it should be marked now as solved
```
