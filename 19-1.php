<?php
if ($argc < 2) {
    die('Usage: ' . $argv[0] . ' <input>' . PHP_EOL);
}
$input = (int)$argv[1];

$hb = 1;
$inp = $input;
while ($inp) {
    $inp >>= 1;
    $hb <<= 1;
}
$result = (($input & ~($hb >> 1)) << 1) | 1;
echo $result . PHP_EOL;