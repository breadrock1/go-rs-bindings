build cbin:
```shell
gcc -Wall -g -O0 cbin/main.c -I. -Ltarget/debug/ -lrslib
```

build gobin
```shell
go build -buildmode=c-shared gobin/main.go
```
