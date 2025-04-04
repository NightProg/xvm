BITS 16
ORG 0x7C00  ; Adresse de chargement en mode réel



start:
    mov ah, 0x0E  ; Fonction BIOS pour afficher un caractère en mode TTY

    mov al, 'H'
    int 0x10
    mov al, 'E'
    int 0x10
    mov al, 'L'
    int 0x10
    mov al, 'L'
    int 0x10
    mov al, 'O'
    int 0x10
    mov al, ' '
    int 0x10
    mov al, 'W'
    int 0x10
    mov al, 'O'
    int 0x10
    mov al, 'R'
    int 0x10
    mov al, 'L'
    int 0x10
    mov al, 'D'
    int 0x10

    lgdt [gdt_pointer]

    jmp done

done:
    jmp $  ; Bloque la CPU ici (fin du programme)

gdt:
    dq 0                ; Descripteur NULL
gdt_code:
    dw 0xFFFF           ; Limite (4 Go)
    dw 0                ; Base (0)
    db 0                ; Base (suite)
    db 0x9A             ; Type (exécutable, en mode protégé)
    db 0xCF             ; Granularité et autres flags
    db 0                ; Base (fin)

gdt_data:
    dw 0xFFFF           ; Limite (4 Go)
    dw 0                ; Base (0)
    db 0                ; Base (suite)
    db 0x92             ; Type (lecture/écriture)
    db 0xCF             ; Granularité
    db 0                ; Base (fin)

gdt_pointer:
    dw gdt_pointer - gdt - 1  ; Taille de la GDT
    dd gdt                    ; Adresse de la GDT


times 510 - ($ - $$) db 0  ; Remplissage jusqu'à 510 octets
dw 0xAA55  ; Signature du bootloader
