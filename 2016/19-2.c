#include <stdio.h>
#include <stdlib.h>

struct _elf {
    int num;
    struct _elf *prev;
    struct _elf *next;
};

void main(int argc, char **argv) {
    int input, i;
    struct _elf *lastElf = NULL;
    struct _elf *firstElf = NULL;
    struct _elf *middleElf = NULL;
    struct _elf *elf = NULL;

    input = atoi(argv[1]);

    firstElf = (struct _elf *)malloc(sizeof(struct _elf));
    firstElf->num = 1;
    lastElf = firstElf;
    for (i = 2; i <= input; i++) {
        elf = (struct _elf *)malloc(sizeof(struct _elf));
        elf->num = i;
        elf->prev = lastElf;
        lastElf->next = elf;

        if (i == ((input / 2) + 1)) { middleElf = elf; }
        lastElf = elf;
    }
    // Make a complete chain
    elf->next = firstElf;
    firstElf->prev = elf;

    elf = firstElf;

    while (elf != elf->next) {
        middleElf->prev->next = middleElf->next;
        middleElf->next->prev = middleElf->prev;

        if (!(input % 2)) {
            middleElf = middleElf->next;
        } else {
            middleElf = middleElf->next->next;
        }
        input--;
        elf = elf->next;
    }

    printf("%d\n", elf->num);
}