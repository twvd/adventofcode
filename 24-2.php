<?php
function findPath(array $startXY, array $endXY) : int {
    global $maze;

    $queue = [$startXY + ['steps' => 0]];
    $beenTo = [];

    while (count($queue)) {
        $step = array_shift($queue);

        foreach ([[-1, 0], [0, -1], [1, 0], [0, 1]] as $dir) {
            $newXY = [
                'x' => $step['x'] + $dir[0],
                'y' => $step['y'] + $dir[1],
                'steps' => $step['steps'] + 1
            ];

            $x = $newXY['x'];
            $y = $newXY['y'];

            if ($maze[$y][$x] == '#') {
                // Blocked
                continue;
            }
            if (isset($beenTo[$x . ',' . $y])) {
                // Already been here
                continue;
            }

            $beenTo[$x . ',' . $y] = true;

            if (($newXY['x'] == $endXY['x']) && ($newXY['y'] == $endXY['y'])) {
                // Reached this goal
                return $newXY['steps'];
            }

            $queue []= $newXY;
        }
    }
}

function findNextGoal(array $startXY, array $goals) : array {
    $lowest = PHP_INT_MAX;
    $lowestg = -1;

    foreach ($goals as $n => $g) {
        $steps = findPath($startXY, $g);
        if ($steps < $lowest) {
            $lowest = $steps;
            $lowestg = $n;
        }
    }

    return [$lowest, $lowestg];
}

// http://docstore.mik.ua/orelly/webprog/pcook/ch04_26.htm
function pc_next_permutation($p, $size) {
    // slide down the array looking for where we're smaller than the next guy
    for ($i = $size - 1; ($i >= 0) && $p[$i] >= $p[$i+1]; --$i) { }

    // if this doesn't occur, we've finished our permutations
    // the array is reversed: (1, 2, 3, 4) => (4, 3, 2, 1)
    if ($i == -1) { return false; }

    // slide down the array looking for a bigger number than what we found before
    for ($j = $size; $p[$j] <= $p[$i]; --$j) { }

    // swap them
    $tmp = $p[$i]; $p[$i] = $p[$j]; $p[$j] = $tmp;

    // now reverse the elements in between by swapping the ends
    for (++$i, $j = $size; $i < $j; ++$i, --$j) {
         $tmp = $p[$i]; $p[$i] = $p[$j]; $p[$j] = $tmp;
    }

    return $p;
}

$maze = file('24.txt');
$maze = array_map(function ($b) { return str_split(trim($b)); }, $maze);

$maxX = count($maze[0]) - 1;
$maxY = count($maze) - 1;

// Find coordinates of goals
$goals = [];
for ($x = 0; $x < $maxX; $x++) {
    for ($y = 0; $y < $maxY; $y++) {
        if (is_numeric($maze[$y][$x])) {
            $goals[$maze[$y][$x]] = ['x' => $x, 'y' => $y];
        }
    }
}

$targets = array_keys($goals);
// Remove 0
sort($targets);
array_shift($targets);

$targetCount = count($targets);

$shortestSteps = PHP_INT_MAX;

// Optimized: once a distance has been found between two points, cache it
$distCache = [];

do {
    $start = $goals[0];
    $startNum = 0;
    $totalSteps = 0;

    foreach ($targets as $goal) {
        if (isset($distCache[$startNum . ',' . $goal])) {
            $steps = $distCache[$startNum . ',' . $goal];
        } else {
            $steps = findPath($start, $goals[$goal]);
            $distCache[$startNum . ',' . $goal] = $steps;
        }
        $totalSteps += $steps;

        //echo $startNum . ' -> ' . $goal . ': ' . $steps . ' (' . $totalSteps . ')' . PHP_EOL;

        $start = $goals[$goal];
        $startNum = $goal;

        if ($totalSteps > $shortestSteps) {
            // No need to go further, we're already over the shortest we've found
            //echo 'STOP' . PHP_EOL;
            continue 2;
        }
    }

    // Back to 0
    $steps = findPath($goals[$goal], $goals[0]);
    //echo $goal . ' -> 0: ' . $steps . PHP_EOL;
    $totalSteps += $steps;
    //echo '-------' . PHP_EOL;

    if ($totalSteps < $shortestSteps) { 
        $shortestSteps = $totalSteps;
        echo '0 -> ' . implode(' -> ', $targets) . ' -> 0 TOTAL: ' . $totalSteps . PHP_EOL;
    }
} while ($targets = pc_next_permutation($targets, $targetCount - 1));

echo 'Answer #2: ' . $shortestSteps . PHP_EOL;