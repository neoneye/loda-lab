; 1,2,6,7,11,15,16,20,24,25,29,30,34,38,39,43,47,48,52,53,57,61,62,66,70,71,75,79,80,84,85,89,93,94,98,102,103,107,108,112

mov $2,7
mov $3,$0
mov $5,1
add $5,$0
lpb $2
  mov $6,$5
  lpb $5
    mov $5,$4
    pow $6,2
  lpe
  mov $0,2
  mov $1,5
  mov $5,1
  lpb $6
    add $0,3
    add $5,$1
    trn $6,$5
  lpe
  mov $2,1
lpe
sub $0,2
add $0,$3
sub $0,2