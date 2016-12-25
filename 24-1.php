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

$start = $goals[0];
$startNum = 0;
unset($goals[0]);

$totalSteps = 0;

while (count($goals)) {
    list($steps, $goal) = findNextGoal($start, $goals);
    echo $startNum . ' -> ' . $goal . ': ' . $steps . PHP_EOL;
    $totalSteps += $steps;

    $start = $goals[$goal];
    $startNum = $goal;
    unset($goals[$goal]);
}
echo 'Answer #1: ' . $totalSteps . PHP_EOL;
