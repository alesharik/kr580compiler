mov hl, 0x83FF
out 2
.NO: mov b, 0b0111111
    mov m, b
.IN: in 0
sub 10
jm .NO
mov b, 0b0000110
mov m, b
jmp .IN
