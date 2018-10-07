Run the binary on the stm32f407 discovery board as per normal. With
the gdb configuration as in this directory, an itm.fifo file will be
written.

Run this command:

```
itmdump -F -f itm.fifo
```

to follow it.
