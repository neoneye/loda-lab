; 0,1,2,3,4,6,8,10,12,14,16,18,20,22,24,26,28,30,32,34,36,38,40,42,44,46,49,52,55,58,61,64,67,70,73,76,79,82,85,88

mov $1,$0
lpb $1
  pow $3,2
  add $3,2
  sub $1,$3
  sub $1,2
  add $0,$1
  sub $1,1
  add $2,2
  add $3,$2
lpe