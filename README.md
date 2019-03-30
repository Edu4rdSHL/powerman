powerman - Power Management framework for Linux written in Rust.

It project is being writen, not ready for use yet.

# Usage

`$ powerman <Minimun level of battery before ation> <Time in seconds to check the battery level> <Action>`

Example:

```
$ powerman 5 60 hibernate # Hibernate the computer when the battery level is less than 5%, the check is done every 60 seconds.
```

If you want to keep it running in the background, just add & at the end of the command, to run in startup, add the desired command to .bashrc, your desktop/WM startup config or enable powerman.service.