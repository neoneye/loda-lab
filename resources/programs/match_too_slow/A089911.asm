; A089911: a(n) = Fibonacci(n) mod 12.
; 0,1,1,2,3,5,8,1,9,10,7,5,0,5,5,10,3,1,4,5,9,2,11,1,0,1,1,2,3,5,8,1,9,10,7,5,0,5,5,10,3,1,4,5,9,2,11,1,0,1,1,2,3,5,8,1,9,10,7,5,0,5,5,10,3,1,4,5,9,2,11,1,0,1,1,2,3,5,8,1,9,10,7,5,0,5,5,10,3,1,4,5,9,2,11,1,0,1,1

mov $1,$0
seq $1,45
mod $1,12
