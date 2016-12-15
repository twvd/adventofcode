<?php
class Disc {
    private $max;
    private $initial;

    public function __construct(int $initial, int $max) {
        $this->max = $max;
        $this->initial = $initial;
    }
    public function isOpenAt(int $time) : bool {
        return ((($this->initial + $time) % $this->max) == 0);
    }
};

function runPuzzle(array $discs) : int {
    $time = 0;
    for ($time = 0; true; $time++) {
        for ($t = 0; $t < count($discs); $t++) {
            $itime = $time + $t + 1;
            if (!$discs[$t]->isOpenAt($itime)) {
                continue 2;
            }
        }
        break;
    }
    return $time;
}

$instructions = file('15.txt');
$discs = [];

// Initialize discs
foreach ($instructions as $line) {
    sscanf($line, 'Disc #%d has %d positions; at time=0, it is at position %d.',
        $disc, $max, $initial);
    $discs []= new Disc($initial, $max);
}

echo 'Answer #1: ' . runPuzzle($discs) . PHP_EOL;

// Add the extra disc for puzzle #2
$discs []= new Disc(0, 11);
echo 'Answer #2: ' . runPuzzle($discs) . PHP_EOL;