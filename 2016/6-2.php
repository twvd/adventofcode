<?php
$data = file_get_contents('6.txt');
preg_match_all('/([a-z]{8})/', $data, $data);
$data = $data[0];

// Initialize letter arrays
$letters = [];
for ($i = 0; $i < 8; $i++) {
    $letters[$i] = [];
}

// Count instances of letters in one run through the file
foreach ($data as $line) {
    for ($i = 0; $i < 8; $i++) {
        $c = substr($line, $i, 1);
        if (!isset($letters[$i][$c])) { $letters[$i][$c] = 0; }
        $letters[$i][$c]++;
    }
}

// Output the end result
for ($i = 0; $i < 8; $i++) {
    // .. one letter difference (asort/arsort) for the second puzzle :)
    asort($letters[$i]);
    echo array_keys($letters[$i])[0];
}
