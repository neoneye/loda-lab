; A112282: a(n) = (-1)^n*(2*n+1) (mod 9).
; 1,6,5,2,0,7,4,3,8,8,3,4,7,0,2,5,6,1,1,6,5,2,0,7,4,3,8,8,3,4,7,0,2,5,6,1,1,6,5,2

mov $1,$0
seq $1,199301 ; (2n+1)*8^n.
mod $1,9
