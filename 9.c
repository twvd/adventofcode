/** Day 9: Marble Mania
 * https://adventofcode.com/2018/day/9
 * 
 * Implemented this in C as linked list
 * Runs part 2 in about 300ms on my machine
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct marble {
    long lValue;
    struct marble *pPrev;
    struct marble *pNext;
};

long *pPlayerScores;
struct marble *pCurrentMarble;
struct marble *pMarbleHeap;

void marbleSeekReverse(int steps) {
    while (steps--) {
        pCurrentMarble = pCurrentMarble->pPrev;
    }
}

void marbleSeekForward(int steps) {
    while (steps--) {
        pCurrentMarble = pCurrentMarble->pNext;
    }
}

void marbleInsert(long value) {
    struct marble *pNewMarble;

    pNewMarble = pMarbleHeap++;
    pNewMarble->lValue = value;
    pNewMarble->pPrev = pCurrentMarble;
    pNewMarble->pNext = pCurrentMarble->pNext;
    pNewMarble->pNext->pPrev = pNewMarble;
    pCurrentMarble->pNext = pNewMarble;
    pCurrentMarble = pNewMarble;
}

void marbleRemove(void) {
    pCurrentMarble->pPrev->pNext = pCurrentMarble->pNext;
    pCurrentMarble->pNext->pPrev = pCurrentMarble->pPrev;
    pCurrentMarble = pCurrentMarble->pNext;
}

int main(int argc, char **argv) {
    int iPlayers, iLastWorth, iTurn;
    long lMaxScore = 0;

    if (argc < 3) {
        printf("Usage: %s <players> <worth of last marble>\n", argv[0]);
        return -1;
    }

    iPlayers = atoi(argv[1]);
    iLastWorth = atoi(argv[2]);

    pPlayerScores = (long *)malloc(iPlayers * sizeof(long));
    memset(pPlayerScores, 0, iPlayers * sizeof(long));

    // Place marble 0
    pCurrentMarble = (struct marble *)malloc(sizeof(struct marble));
    pCurrentMarble->lValue = 0;
    pCurrentMarble->pPrev = pCurrentMarble;
    pCurrentMarble->pNext = pCurrentMarble;

    // Pre-allocate all marbles for speed
    // (this shaves off half of the execution time compared to mallocing each marble)
    pMarbleHeap = (struct marble *)malloc(iLastWorth * sizeof(struct marble));

    for (iTurn = 1; iTurn <= iLastWorth; iTurn++) {
        if ((iTurn % 23) == 0) {
            int iPlayerNum = (iTurn % iPlayers);
            marbleSeekReverse(7);

            pPlayerScores[iPlayerNum] += iTurn + pCurrentMarble->lValue;
            if (pPlayerScores[iPlayerNum] > lMaxScore) {
                lMaxScore = pPlayerScores[iPlayerNum];
            }

            marbleRemove();
        } else {
            marbleSeekForward(1);
            marbleInsert(iTurn);
       }
    }

    printf("Answer: %ld\n", lMaxScore);
}