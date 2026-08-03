function fib(n@u32) ~> @u32{
    if (n == 0) || (n == 1){
        return n;
    }
    else {
        return fib(n-1) + fib(n-2);
    }
}

function main(){
    let x@u32 = fib(10);
    let power@u32 = 10;
    let res@u32 = 1;
    while power != 0{
        res *= 2;
        power --;
    }
    print(x + res);
}
