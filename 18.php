<?php
$trapPatterns = [
    '^^.', '.^^', '^..', '..^'
];

function calcNextRow(string $row) : string {
    global $trapPatterns, $safe;

    $out = '';
    for ($i = 0; $i < strlen($row); $i++) {
        // Optimized: the trap check is simply if (L != R). C can be ignored.
        $l = ($i == 0 ? '.' : $row{$i - 1});
        //$c = $row{$i};
        $r = ($i >= (strlen($row) - 1) ? '.' : $row{$i + 1});

        //if (in_array($l.$c.$r, $trapPatterns)) {
        if ($l != $r) {
            $out .= '^';
        } else {
            $out .= '.';
            $safe++;
        }
    }
    return $out;
}

$row = trim(file_get_contents('18.txt'));

// Count safe spots in initial row as base for the counter
$safe = substr_count($row, '.');

for ($i = 1; $i < 400000; $i++) {
    $row = calcNextRow($row);

    if ($i == 39) {
        echo 'Answer #1: ' . $safe . PHP_EOL;
    }
}
echo 'Answer #2: ' . $safe . PHP_EOL;