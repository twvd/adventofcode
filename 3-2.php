<?php
	$instructions = file('3-1.txt');
	
	$count = 0;
	$not = 0;
	$triangles = [];
	
	// Rotate triangles in place
	for ($lineCount = 0; $lineCount < count($instructions); $lineCount += 3) {
		list($a, $b, $c) = sscanf($instructions[$lineCount], '%d %d %d');
		$triangles[$lineCount] = [$a];
		$triangles[$lineCount + 1] = [$b];
		$triangles[$lineCount + 2] = [$c];
		list($a, $b, $c) = sscanf($instructions[$lineCount + 1], '%d %d %d');
		$triangles[$lineCount] []= $a;
		$triangles[$lineCount + 1] []= $b;
		$triangles[$lineCount + 2] []= $c;
		list($a, $b, $c) = sscanf($instructions[$lineCount + 2], '%d %d %d');
		$triangles[$lineCount] []= $a;
		$triangles[$lineCount + 1] []= $b;
		$triangles[$lineCount + 2] []= $c;
	}

	foreach ($triangles as $trig) {
		list($a, $b, $c) = $trig;
		
		if (($a + $b > $c) && ($a + $c > $b) && ($b + $c > $a)) {
			$count++;
		} else {
			$not++;
		}
	}
	
	echo $count . PHP_EOL;
	echo $not . PHP_EOL;