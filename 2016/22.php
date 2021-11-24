<?php
function printTable(array $nodes) {
    $x = -1;

    echo '   ';
    for ($y = 0; $y < 29; $y++) {
        printf('%02d ', $y);
    }
    echo PHP_EOL . '  ' . str_repeat('-', (3 * 30));

    foreach ($nodes as $node) {
        if ($node['x'] != $x) {
            echo PHP_EOL . sprintf('%02d|', $node['x']);
            $x = $node['x'];
        }

        if ($node['used'] > 100) { echo '###'; }
        else if ($node['used'] == 0) { echo '   '; }
        else if (($node['x'] == 0) && ($node['y'] == 0)) { echo '[G]'; }
        else if (($node['x'] == 0) && ($node['y'] == 28)) { echo '[*]'; }
        else { echo '[_]'; }
    }
    echo PHP_EOL;
}

$file = file_get_contents('22.txt');
$nodes = [];
foreach (file('22.txt') as $line) {
    if (!preg_match('/node-x([0-9]*)-y([0-9]*)\s*([0-9]*)T\s*([0-9]*)T\s*([0-9]*)T/i', $line, $matches)) {
        continue;
    }

    $node = [
        'x' => $matches[1],
        'y' => $matches[2],
        'size' => $matches[3],
        'used' => $matches[4],
        'avail' => $matches[5]
    ];
    $nodes []= $node;
}

$count = 0;

foreach ($nodes as $node1) {
    if ($node1['used'] == 0) { continue; }

    foreach ($nodes as $node2) {
        if ($node1 == $node2) { continue; }

        if ($node1['used'] <= $node2['avail']) {
            $count++;
        }
    }
}

echo 'Answer #1: ' . $count . PHP_EOL . PHP_EOL;
printTable($nodes);
