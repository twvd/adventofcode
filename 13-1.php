<?php
// Pass puzzle input on the command line
$input = $argv[1];

define('MAP_SIZE', 50);
define('TARGET_X', 31);
define('TARGET_Y', 39);

// Map generator function
function generateMap(int $seed) : array {
    $map = [];

    for ($y = 0; $y < MAP_SIZE; $y++) {
        $map[$y] = [];

        for ($x = 0; $x < MAP_SIZE; $x++) {
            $item = ($x * $x + 3 * $x + 2 * $x * $y + $y + $y * $y);
            $item += $seed;

            // The odd/even bit calculation can be solved by parity
            $map[$y][$x] = calcParity($item);
        }
    }

    return $map;
}

// Function to print a map (and optionally route) to the console
function printMap(array $map, array $route = null) {
    do {
        $map[$route['y']][$route['x']] = 2;
        $route = isset($route['backtrack']) ? $route['backtrack'] : null;
    } while ($route != null);

    echo '   ';
    for ($x = 0; $x < MAP_SIZE; $x++) {
        echo (int)($x / 10);
    }
    echo PHP_EOL . '   ';
    for ($x = 0; $x < MAP_SIZE; $x++) {
        echo ($x % 10);
    }
    echo PHP_EOL . PHP_EOL;
    for ($y = 0; $y < MAP_SIZE; $y++) {
        echo sprintf('%02d ', $y);
        for ($x = 0; $x < MAP_SIZE; $x++) {
            if ($map[$y][$x] === 2) echo 'O';
            else echo ($map[$y][$x] ? '#' : '.');
        }
        echo PHP_EOL;
    }
}

// Naive parity calculation
// http://graphics.stanford.edu/~seander/bithacks.html#ParityNaive
function calcParity(int $num) : bool {
    $parity = false;
    while ($num) {
        $parity = !$parity;
        $num = $num & ($num - 1);
    }
    return $parity;
}

// Function to calculate next possible steps from a location
function calcNextPossibleSteps($map, $state) : array {
    $result = [];
    $currentX = $state['x'];
    $currentY = $state['y'];

    foreach ([-1, 1] as $direction) {
        if (($currentY + $direction) >= 0) {
            if (!$map[$currentY + $direction][$currentX]) {
                $result []= [
                    'x' => $currentX,
                    'y' => ($currentY + $direction)
                ];
            }
        }
        if (($currentX + $direction) >= 0) {
            if (!$map[$currentY][$currentX + $direction]) {
                $result []= [
                    'x' => ($currentX + $direction),
                    'y' => $currentY
                ];
            }
        }
    }

    return $result;
}

// Function to check the history if we've been at certain coordinates already
function beenToCoords($x, $y) {
    global $history;

    $hash = $x.','.$y;

    if (isset($history[$hash])) { return true; }
    $history[$hash] = true;
    return false;
}

$map = generateMap($input);

// Start queue with the starting position
$queue = [['x' => 1, 'y' => 1, 'depth' => 0]];
$history = [];
$maxDepth = 0;
$calculations = 0;

// Using a BFS-algorithm with history checking again for this puzzle
while (count($queue)) {
    $position = array_shift($queue);
    foreach (calcNextPossibleSteps($map, $position) as $possibility) {
        $calculations++;

        if (beenToCoords($possibility['x'], $possibility['y'])) {
            continue;
        }

        $possibility['depth'] = $position['depth'] + 1;
        $possibility['backtrack'] = $position;

        $queue []= $possibility;

        if (($possibility['x'] == TARGET_X) && ($possibility['y'] == TARGET_Y)) {
            printMap($map, $possibility);
            echo 'Steps: ' . ($position['depth'] + 1) . PHP_EOL;
            echo 'Calculations: ' . $calculations . PHP_EOL;
            exit();
        }

        if ($possibility['depth'] > $maxDepth) {
            $maxDepth = $possibility['depth'];
        }
    }
}