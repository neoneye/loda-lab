; 2,6,8,12,14,16,18,20,22,24,26,28,30,32,34,36,38,40,42,44,46,48,50,52,56,58,60,62,64,66,68,70,72,74,76,78,80,82,84,86

mul $0,2
mov $1,$0
mov $2,$3
lpb $1
  add $0,2
  sub $1,$2
  add $2,2
  div $1,6
lpe
add $0,2