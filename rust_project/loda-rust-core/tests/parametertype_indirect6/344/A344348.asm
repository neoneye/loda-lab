; A344348: a(n) = floor(frac(e * n) * n).
; Submitted by PDW
; 0,0,0,0,3,2,1,0,5,4,1,9,7,4,0,11,7,3,16,12,7,1,17,11,5,23,17,10,3,24,16,8,31,23,14,4,30,21,11,0,29,18,7,38,26,14,1,35,22,9,45,32,18,3,42,27,12,53,38,22,5,49,33,15,62,44,26,8,57,38,19,70

mov $1,$0
add $$1,2
mul $1,$0
seq $1,22843 ; Beatty sequence for e: a(n) = floor(n*e).
mod $1,$0
mov $0,$1
