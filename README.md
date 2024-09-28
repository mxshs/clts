# CLTS
Alias your shell commands with a concise config file. Probably useless though. I might add generation of proper shell aliases, which will make this useful.
As of now you run daemon binary as systemd service and then alias client binary to some short name (like `r`). Daemon will create a unix socket at tmp/clts.sock, client will use it communicate.
If you face issues, you should delete the socket and restart the created service.

## Config structure
```
[groupName1]
alias1 -> actualCommand1
alias2 -> actualCommand2
[groupName2]
alias3 -> actualCommand3
```

## Usage example
Say you have aliased the client binary to `r`, and you are using the following config:
```
[test]
f -> find
```
Then, if you run `r f .`, you should see all files in the current directory.
Do not forget, that if you want to pass flags, you have to add `--` between the command and the flag (to prevent flags from being passed to clts binary).
