_main:
	mov r0, 21
	mov r1, af
	mul r2, r0, r1
	bne r2, 0, skip
	mul r2, r2, r0 ;example example example
	
skip:
	add r1, 21
	add r4, r1, r0