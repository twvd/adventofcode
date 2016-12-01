<?php
	$instructions = explode(',', file_get_contents('1-1.txt'));
	$instructions = array_map('trim', $instructions);
	
	$direction = 0;
	$posY = 0;
	$posX = 0;
	
	foreach ($instructions as $i) {
		list($dir, $steps) = sscanf($i, '%c%d');
		if ($dir == 'L') {
			if (--$direction < 0) 
				$direction = 3;
		} else {
			$direction = ($direction + 1) % 4;			
		}
		
		if ($direction == 0) { $posY -= $steps; }
		else if ($direction == 1) { $posX += $steps; }		
		else if ($direction == 2) { $posY += $steps; }
		else if ($direction == 3) { $posX -= $steps; }
		
		echo $dir . '(' . $direction . '): ' . $steps . PHP_EOL;		
	}
	echo $posY . ' - ' . $posX . PHP_EOL;
	echo abs($posY) + abs($posX) . PHP_EOL;