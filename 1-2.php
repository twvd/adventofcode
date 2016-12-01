<?php
	$instructions = explode(',', file_get_contents('1-2.txt'));
	$instructions = array_map('trim', $instructions);
	
	$direction = 0;
	$posY = 0;
	$posX = 0;
	$visited = [];
	
	foreach ($instructions as $i) {
		list($dir, $steps) = sscanf($i, '%c%d');
		if ($dir == 'L') {
			if (--$direction < 0) 
				$direction = 3;
		} else {
			$direction = ($direction + 1) % 4;			
		}
		
		while ($steps--) {
			if ($direction == 0) { $posY -= 1; }
			else if ($direction == 1) { $posX += 1; }		
			else if ($direction == 2) { $posY += 1; }
			else if ($direction == 3) { $posX -= 1; }
		
			echo $dir . '(' . $direction . '): ' . $steps . PHP_EOL;		
			echo $posY . ',' . $posX . PHP_EOL;
			if (isset($visited[$posY . ',' . $posX])) {
				echo $posY . ',' . $posX . PHP_EOL;
				echo abs($posY) + abs($posX) . PHP_EOL;
				exit();
			}
			$visited[$posY . ',' . $posX] = 1;
		}
	}