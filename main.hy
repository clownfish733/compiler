function fib(n@u32) ~> @u32{
    if (n == 0) || (n == 1){
        return n;
    }
    else {
        return fib(n-1) + fib(n-2);
    }
}

function main(){
    let x@u32 = fib(4);
    let res@u32 = 1;
    let power@u32 = 10;
    while power != 0{
        res *= 2;
        power --;
    }
    let z@u32 = 7 * res + 2;
    print(x + res + z);
    return;
}

































def fib(n):
    if n == 0 or n == 1:
        return n
    else:
        return fib(n-1) + fib(n-2)


x = fib(4)
res = 1
power = 10
while power:
    res *= 2
    power -= 1

z = 7 * power + 2
print(z + x + power)






















