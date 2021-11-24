<?php
$instructions = file('3-1.txt');

$count = 0;
$not = 0;

foreach ($instructions as $line) {
    list($a, $b, $c) = sscanf($line, '%d %d %d');

    if (($a + $b > $c) && ($a + $c > $b) && ($b + $c > $a)) {
        $count++;
    } else {
        $not++;
    }
}

echo $count . PHP_EOL;