<?php
function isEven(int $i) : bool {
    return ($i % 2) == 0;
}

function dragonCurve(string $a) : string {
    $b = $a;
    $b = strrev($a);
    $b = str_replace(['0', '1', 'A'], ['A', '0', '1'], $b);
    return $a.'0'.$b;
}

function checksum(string $s) : string {
    $checksum = '';

    for ($i = 0; $i < strlen($s); $i += 2) {
        $part = substr($s, $i, 2);
        if ($part{0} == $part{1}) {
            $checksum .= '1';
        } else {
            $checksum .= '0';
        }
    }

    if (isEven(strlen($checksum))) {
        $checksum = checksum($checksum);
    }
    return $checksum;
}

if ($argc < 3) {
    echo 'Usage: ' . $argv[0] . ' <length> <input>' . PHP_EOL;
    exit();
}
$input = $argv[2];
$length = $argv[1];

while (strlen($input) < $length) {
    $input = dragonCurve($input);
}
$input = substr($input, 0, $length);
echo 'Expansion complete, calculating checksum...' . PHP_EOL;
echo 'Checksum: ' . checksum($input) . PHP_EOL;