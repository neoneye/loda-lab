; A172338 a(n) = floor(n*(sqrt(5)+sqrt(3))).
; 0,3,7,11,15,19,23,27,31,35,39,43,47,51,55,59,63,67,71,75,79,83,87,91,95,99,103,107,111,115,119,123,MISMATCH
; 32 terms correct.

mul $0,4
trn $3,3
trn $0,1
pow $3,0
mov $1,$0
mul $1,$3
