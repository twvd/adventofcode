<?php
//    x->
//    0123
// y 0xxxx
//   1xxxx
//   2xxxx
//   3xxxx
$directions = [
    // x, y
    [0, -1],    // Up
    [0, 1],     // Down
    [-1, 0],
    [1, 0]
];
$directionsT = [ 'U', 'D', 'L', 'R' ];

function getPossibleDirs(string $hash) : array {
    $result = [];

    for ($i = 0; $i < 4; $i++) {
        if (is_numeric($hash{$i}) || ($hash{$i} == 'a')) {
            continue;
        }
        $result []= $i;
    }
    return $result;
}

if ($argc < 2) {
    echo 'Usage: ' . $argv[0] . ' <input>' . PHP_EOL;
    exit();
}
$input = $argv[1];

$initialState = [
    'x' => 0,
    'y' => 0,
    'str' => ''
];
$queue = [$initialState];
$longest = 0;

while (count($queue)) {
    $state = array_shift($queue);
    $hash = md5($input . $state['str']);
    if (($state['x'] == 3) && ($state['y'] == 3)) {
        if (strlen($state['str']) > $longest) {
            if ($longest == 0) {
                echo 'Answer #1: ' . $state['str'] . PHP_EOL;
            }
            $longest = strlen($state['str']);
        }
        continue;
    }

    foreach (getPossibleDirs($hash) as $direction) {
        $newState = $state;
        $newState['x'] += $directions[$direction][0];
        $newState['y'] += $directions[$direction][1];
        $newState['str'] .= $directionsT[$direction];

        if (($newState['x'] < 0) || ($newState['y'] < 0) || ($newState['x'] > 3) || ($newState['y'] > 3)) {
            continue;
        }

        $queue []= $newState;
    }
}
if ($longest == 0) {
    echo 'Impossible to exit - input is invalid' . PHP_EOL; 
} else {
    echo 'Answer #2: ' . $longest . PHP_EOL;
}