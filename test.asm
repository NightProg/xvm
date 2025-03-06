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

    jmp done

done:
    jmp $  ; Bloque la CPU ici (fin du programme)

times 510 - ($ - $$) db 0  ; Remplissage jusqu'à 510 octets
dw 0xAA55  ; Signature du bootloader
