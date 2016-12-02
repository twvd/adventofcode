<?php
	$instructions = file('2-1.txt');
	
	$keypad = [
		[0,   0,   1,   0,   0],
		[0,   2,   3,   4,   0],
		[5,   6,   7,   8,   9],
		[0, 'A', 'B', 'C',   0],
		[0,   0, 'D',   0,   0]
	];
	
	$pin = '';
	
	$x = 0;
	$y = 2;
	
	foreach ($instructions as $line) {	
		foreach (str_split(trim($line)) as $direction) {
			$newX = $x;
			$newY = $y;
			
			if ($direction == 'L') { $newX--; }
			else if ($direction == 'R') { $newX++; }
			else if ($direction == 'U') { $newY--; }
			else if ($direction == 'D') { $newY++; }
			
			if (($newX >= 0) && ($newY >= 0) && ($newX < 5) && ($newY < 5)) {
				if ($keypad[$newY][$newX] !== 0) {
					$x = $newX;
					$y = $newY;
				}
			}
			
			//echo $direction . ': ' . $x . ',' . $y . ': ' . $keypad[$y][$x] . PHP_EOL;
		}
		
		//echo 'EOL: ' . $x . ',' . $y . ': ' . $keypad[$y][$x] . PHP_EOL;
		$pin .= $keypad[$y][$x];
	}
	
	echo 'PIN: ' . $pin . PHP_EOL;