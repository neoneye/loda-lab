; 1,2,3,4,5,6,7,8,10,12,14,16,18,20,22,25,28,31,34,37,40,43,46,50,54,58,62,66,70,74,79,84,89,94,99,104,109,115,121,127

mov $2,$0
mov $3,$0
add $3,1
lpb $3
  sub $3,1
  mov $0,$2
  sub $0,$3
  mul $0,4
  mov $4,$0
  mul $0,4
  add $4,$0
  div $4,3
  div $4,49
  add $4,1
  add $1,$4
lpe
mov $0,$1

; template 22790
; mutation: InsertInstructionWithConstant
