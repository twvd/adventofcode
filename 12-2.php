<?php
$code = file('12.txt');

// Initial state of register file
$registers = [
    'a' => 0,
    'b' => 0,
    'c' => 1,
    'd' => 0
];
$cycles = 0;

for ($ptr = 0; $ptr < count($code); $ptr++) {
    // Load and split instruction
    list($instr, $x, $y) = explode(' ', trim($code[$ptr]) . ' ');

    if ($instr == 'cpy') {
        // First operand: source value
        // Second operand: target register
        // CPY can have a register or number as first operand (source)
        $value = (is_numeric($x) ? $x : $registers[$x]);
        $registers[$y] = $value;
    }
    else if ($instr == 'jnz') {
        // First operand: value/register to check if not zero
        // Second operand: program counter offset to jump
        // JNZ can have a number or register as first operand
        $cmp = (is_numeric($x) ? $x : $registers[$x]);
        if ($cmp != 0) {
            // Decrement the target by 1, because the for loop will increment afterwards
            $ptr += ($y - 1);
        }
    }
    else if ($instr == 'inc') { $registers[$x]++; }
    else if ($instr == 'dec') { $registers[$x]--; }

    $cycles++;
}

echo 'Done, cycles: ' . $cycles . PHP_EOL;
echo 'A: ' . $registers['a'] . ' B: ' . $registers['b'] . ' C: ' . $registers['c'] . ' D: ' . $registers['d'] . PHP_EOL;