<?php
function runInstruction(array $input, string $instr) : array {
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
    return $input;
}

function scramble(string $input) : string {
    global $instructions;

    $input = str_split($input);

    foreach ($instructions as $instr) {
        $input = runInstruction($input, $instr);
    }
    return implode('', $input);
}

// http://stackoverflow.com/questions/2617055/how-to-generate-all-permutations-of-a-string-in-php
function permute($arg) {
    $array = is_string($arg) ? str_split($arg) : $arg;
    if(1 === count($array))
        return $array;
    $result = array();
    foreach($array as $key => $item)
        foreach(permute(array_diff_key($array, array($key => $item))) as $p)
            $result[] = $item . $p;
    return $result;
}

$instructions = file('21.txt');
$instructions = array_map('trim', $instructions);

$permutations = permute('abcdefgh');

foreach ($permutations as $guess) {
    $scr = scramble($guess);
    if ($scr == $argv[1]) {
        echo $guess . PHP_EOL;
        exit();
    }
}