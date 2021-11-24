<?php
class Disc {
    private $max;
    private $initial;

    public function __construct(int $initial, int $max) {
        $this->max = $max;
        $this->initial = $initial;
    }
    public function getPosition(int $time) : int {
        return (($this->initial + $time) % $this->max);
    }
    public function isOpenAt(int $time) : bool {
        return ($this->getPosition($time) == 0);
    }
    public function getReturnToZero(int $time) : int {
        return ($this->max - $this->getPosition($time));
    }
};

function runPuzzle(array $discs) {
    $rounds = 0;
    $tStart = microtime(true);

    // Optimization: start at the first time the first disc hits 0
    $time = $discs[0]->getReturnToZero(1);

    while (true) {
        $rounds++;
        $returnToZero = 1;

        for ($t = 0; $t < count($discs); $t++) {
            $itime = $time + $t + 1;
            if (!$discs[$t]->isOpenAt($itime)) {
                // Optimization: we only have to consider the next time that all the discs that
                // are at 0 now are at 0 again, so skip forward in time.
                $time += $returnToZero;
                continue 2;
            }

            $returnToZero *= $discs[$t]->getReturnToZero($itime);
        }
        break;
    }

    echo 'Answer: ' . $time . ' (found in ' . $rounds . ' rounds, ' . ((microtime(true) - $tStart) * 1000) . ' ms)' . PHP_EOL;
}

$instructions = file('15.txt');
$discs = [];

// Initialize discs
foreach ($instructions as $line) {
    sscanf($line, 'Disc #%d has %d positions; at time=0, it is at position %d.',
        $disc, $max, $initial);
    $discs []= new Disc($initial, $max);
}

echo 'Puzzle #1: ';
runPuzzle($discs);

// Add the extra disc for puzzle #2
$discs []= new Disc(0, 11);
echo 'Puzzle #2: ';
runPuzzle($discs);