; 0,1,9,36,100,225,441,784,1296,2025,2026,2034,2061,2125,2250,2466,2809,3321,4050,5050,5058,5085,5149,5274,5490,5833,6345,7074,8074,9405,9432,9496,9621,9837,10180,10692,11421,12421,13752,15480

mov $4,$0
mov $3,$0
lpb $3
  sub $3,1
  mov $0,$4
  sub $0,$3
  mov $2,$0
  lpb $0
    mod $0,10
    div $2,10
    add $2,$0
  lpe
  pow $2,3
  add $1,$2
lpe
mov $0,$1
