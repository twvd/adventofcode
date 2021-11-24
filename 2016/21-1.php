<?php
function runInstruction(array $input, string $instr) : array {
    echo implode('', $input);

    if (sscanf($instr, 'swap position %d with position %d', $p1, $p2)) {
        $l = $input[$p1];
        $r = $input[$p2];
        $input[$p2] = $l;
        $input[$p1] = $r;
    } else if (sscanf($instr, 'swap letter %c with letter %c', $c1, $c2)) {
        $i1 = array_search($c1, $input);
        $i2 = array_search($c2, $input);
        $a = $input[$i1];
        $input[$i1] = $input[$i2];
        $input[$i2] = $a;
    } else if (sscanf($instr, 'rotate left %d', $count)) {
        while ($count--) { array_push($input, array_shift($input)); }
    } else if (sscanf($instr, 'rotate right %d', $count)) {
        while ($count--) { array_unshift($input, array_pop($input)); }
    } else if (sscanf($instr, 'rotate based on position of letter %c', $char)) {
        $count = array_search($char, $input);
        if ($count >= 4) { $count++; }
        $count++;
        while ($count--) { array_unshift($input, array_pop($input)); }
    } else if (sscanf($instr, 'reverse positions %d through %d', $i1, $i2)) {
        $part = array_reverse(array_slice($input, $i1, ($i2 - $i1) + 1));
        array_splice($input, $i1, ($i2 - $i1) + 1, $part);
    } else if (sscanf($instr, 'move position %d to position %d', $i1, $i2)) {
        $c = $input[$i1];
        unset($input[$i1]);
        $input = array_values($input);
        array_splice($input, $i2, 0, $c);
    } else {
        die('Unknown instruction: ' . $instr);
    }
    echo ' | ' . implode('', $input) . ' | ' . $instr . PHP_EOL;
    return $input;
}

$instructions = file('21.txt');
$instructions = array_map('trim', $instructions);
$input = str_split($argv[1]);

foreach ($instructions as $instr) {
    $input = runInstruction($input, $instr);
}
echo implode('', $input) . PHP_EOL;