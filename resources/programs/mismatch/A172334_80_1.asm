; A172334 Floor(n*(sqrt(13)+sqrt(3))).
; 0,5,10,16,21,26,32,37,42,48,53,58,64,69,74,80,85,90,96,101,106,112,117,122,128,133,138,snip,410,416,421,MISMATCH
; 80 terms correct.

mov $3,$4
dif $2,2
mul $0,2
seq $1,40
pow $2,2
mov $3,9
trn $3,1
add $1,4
add $1,2
mul $1,$0
div $1,3
