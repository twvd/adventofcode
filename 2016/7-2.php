<?php
// Function to get ABAs from an IPv7 segment
function getABAs($segment) {
    $aba = [];

    for ($i = 0; $i < (strlen($segment) - 2); $i++) {
        $a = substr($segment, $i, 1);
        $b = substr($segment, $i + 1, 1);
        $c = substr($segment, $i + 2, 1);

        if ($a != $c) { continue; }
        if ($a == $b) { continue; }

        $aba []= $a . $b;
    }

    return $aba;
}

// BABs are just inverted ABAs, after all
function getBABs($segment) {
    $aba = getABAs($segment);
    return array_map('strrev', $aba);
}

$ipsWithSsl = 0;

foreach (file('7.txt') as $line) {
    preg_match_all('/[a-z]+/', $line, $matches);

    // Text between square brackets is every odd match, so we can just switch it every iteration
    $isSquare = false;

    $abas = [];
    $babs = [];

    foreach ($matches[0] as $segment) {
        if ($isSquare) {
            $babs = array_merge($babs, getBABs($segment));
        } else {
            $abas = array_merge($abas, getABAs($segment));
        }
        $isSquare = !$isSquare;
    }

    // Check the matches by intersecting the arrays. Not every ABA needs to have a matching BAB,
    // but at least one.
    if ((count($abas) > 0) && (count(array_intersect($abas, $babs)) > 0)) {
        $ipsWithSsl++;
    }
}

echo $ipsWithSsl . PHP_EOL;