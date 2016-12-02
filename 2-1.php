<?php
	$instructions = file('2-1.txt');
	
	$keypad = [
		[1, 2, 3],
		[4, 5, 6],
		[7, 8, 9]
	];
	
	$pin = '';
	
	$x = 1;
	$y = 1;
	
	foreach ($instructions as $line) {	
		foreach (str_split(trim($line)) as $direction) {			
			if ($direction == 'L') { if ($x > 0) { $x--; } }
			else if ($direction == 'R') { if ($x < 2) { $x++; } }
			else if ($direction == 'U') { if ($y > 0) { $y--; } }
			else if ($direction == 'D') { if ($y < 2) { $y++; } }
			//echo $direction . ': ' . $x . ',' . $y . ': ' . $keypad[$y][$x] . PHP_EOL;
		}
		
		echo 'EOL: ' . $x . ',' . $y . ': ' . $keypad[$y][$x] . PHP_EOL;
		$pin .= $keypad[$y][$x];
	}
	
	echo 'PIN: ' . $pin . PHP_EOL;