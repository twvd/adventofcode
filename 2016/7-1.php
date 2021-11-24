<?php
// Function to check if a certain IPv7 address segment has ABBA
function hasABBA($segment) {
    if (strlen($segment) < 4) { return false; }

    for ($i = 2; $i < strlen($segment); $i++) {
        $left = substr($segment, $i - 2, 2);
        $right = substr($segment, $i, 2);

        if ($left == $right) { continue; }

        if ($left == strrev($right)) {
            return true;
        }
    }

    return false;
}

$ipsWithTls = 0;

foreach (file('7.txt') as $line) {
    preg_match_all('/[a-z]+/', $line, $matches);

    // Text between square brackets is every odd match, so we can just switch it every iteration
    $isSquare = false;
    $abbaFound = false;

    foreach ($matches[0] as $segment) {
        $hasABBA = hasABBA($segment);

        if ($isSquare && $hasABBA) {
            continue 2;
        } else if ($hasABBA) {
            $abbaFound = true;
        }

        $isSquare = !$isSquare;
    }

    if ($abbaFound) { $ipsWithTls++; }
}

echo $ipsWithTls . PHP_EOL;