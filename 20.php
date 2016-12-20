<?php
$blockList = file('20.txt');

// Put blocklist in array by [min, max]
$blockList = array_map(function ($b) { return explode('-', trim($b)); }, $blockList);

// Sort blocklist by minimums
usort($blockList, function ($a, $b) { return ($a[0] > $b[0]); });

// Combine the ranges in the blocklist
$newBl = [$blockList[0]];
$b = 0;
for ($i = 0; $i < count($blockList); $i++) {
    if (($blockList[$i][0] >= $newBl[$b][0]) && ($blockList[$i][1] <= $newBl[$b][1])) {
        // Within the range, skip
        continue;
    }
    if (($blockList[$i][0] - 1) > $newBl[$b][1]) {
        // New entry
        $newBl[++$b] = $blockList[$i];
        continue;
    }
    if ($blockList[$i][1] > $newBl[$b][1]) {
        // New max
        $newBl[$b][1] = $blockList[$i][1];
    }
}

echo 'Answer #1: ' . ($newBl[0][1] + 1) . PHP_EOL;

$totalAllowed = 0;
for ($i = 1; $i < count($newBl); $i++) {
    $allowed = ($newBl[$i][0] - $newBl[$i - 1][1]) - 1;
    $totalAllowed += $allowed;
}

// Add end of IP range to last blocklist entry aswell
$totalAllowed += (4294967295 - $newBl[count($newBl) - 1][1]);

echo 'Answer #2: ' . $totalAllowed . PHP_EOL;