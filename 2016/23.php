<?php
// Map for tgl instruction
$tglOpcodes = [
    'inc' => 'dec',
    'jnz' => 'cpy',
    'dec' => 'inc',
    'tgl' => 'inc',
    'cpy' => 'jnz'
];

// Initial state of register file
$registers = [
    'a' => $argv[1],
    'b' => 0,
    'c' => 0,
    'd' => 0
];
$cycles = 0;

$code = file_get_contents('23.txt');

// Replace MUL-routine by instruction
$code = str_replace('inc a
dec c
jnz c -2
dec d
jnz d -5',
'mul b d
nop
nop
nop
nop',
$code);

$code = explode("\n", trim($code));
$code = array_map(function ($a) { return explode(' ', trim($a)); }, $code);

for ($ptr = 0; $ptr < count($code); $ptr++) {
    // Load and split instruction
    $instr = $code[$ptr][0];
    if (isset($code[$ptr][1])) { $x = $code[$ptr][1]; } else { unset($x); }
    if (isset($code[$ptr][2])) { $y = $code[$ptr][2]; } else { unset($y); }

    //echo $cycles . ':' . $ptr . ':' . $instr . ' ' . $x . ' ' . (isset($y) ? $y : '') . ' (' . implode(':', $registers) . ')' . PHP_EOL;

    if ($instr == 'cpy') {
        // First operand: source value
        // Second operand: target register
        // CPY can have a register or number as first operand (source)
        $value = (is_numeric($x) ? $x : $registers[$x]);
        if (!is_numeric($y)) {
            $registers[$y] = $value;
        }
    }
    else if ($instr == 'jnz') {
        // First operand: value/register to check if not zero
        // Second operand: program counter offset to jump
        // JNZ can have a number or register as first operand
        $cmp = (is_numeric($x) ? $x : $registers[$x]);
        $y = (is_numeric($y) ? $y : $registers[$y]);
        if ($cmp != 0) {
            // Decrement the target by 1, because the for loop will increment afterwards
            $ptr += ($y - 1);
        }
    }
    else if ($instr == 'inc') { $registers[$x]++; }
    else if ($instr == 'dec') { $registers[$x]--; }
    else if ($instr == 'tgl') {
        $x = (is_numeric($x) ? $x : $registers[$x]);
        $p = ($ptr + $x);       // Position in the program to toggle
        if ($p < count($code)) {
            $code[$p][0] = $tglOpcodes[$code[$p][0]];
        }
    }
    else if ($instr == 'nop') { /* nop! */ }
    else if ($instr == 'mul') {
        $registers['a'] = ($registers[$x] * $registers[$y]);
        $registers['c'] = 0;
        $registers['d'] = 0;
    }
    else { die('Invalid instruction: ' . $instr); }

    $cycles++;
}

echo 'Done, cycles: ' . $cycles . PHP_EOL;
echo 'A: ' . $registers['a'] . ' B: ' . $registers['b'] . ' C: ' . $registers['c'] . ' D: ' . $registers['d'] . PHP_EOL;