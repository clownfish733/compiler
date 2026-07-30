# Plan for custom compiler for custom language

## Requirements
- converts code from a **.hy** file to a **.s** file which can be compiled and run as follows

```bash
as main.s -o main.o
gcc -o main main.o -nostdlib -static
./main
```

- Is statically type language
- requires a main function

## Stages of implementation

### Initial Implementation

    **Types**
    - u32
    



## Grammar

