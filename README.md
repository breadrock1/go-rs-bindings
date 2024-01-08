## Research bindings generating for Rust and Go

There is simple project with researching of bindings generating for Rust and Go code bases. I'll update this project with more complex cases of export functions: structures 
 - sharing structures by pointers and e.t.c;
 - invoking external callbacks and functions;
 - supporting invoking of async callbacks and functions.

### How build and test bindings for C and Golang

build cbin:
```shell
gcc -Wall -g -O0 cbin/main.c -I. -Ltarget/debug/ -lrslib
```

build gobin
```shell
go build -buildmode=c-shared gobin/main.go
```
