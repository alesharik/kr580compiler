mov d, 0x55
mov a, 0x65
add d
mov de, 0x8251
mov [de], a
mov d, 0x0E
mov a, 0x1F
sub d
mov d, a
mov m, d
mov de, 0x8251
mov a, [de]
mov b, 0x01
add b
mov [de], a
mov d, m
mov a, d
mov m, d
mov de, 0x8252
mov [de], a
mov de, 0x8251
mov a, [de]
mov d, m
add d
mov b, a
