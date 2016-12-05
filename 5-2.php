<?php
	$code = $argv[1];
	
	$i = 0;
	$password = [];
	
	while (count($password) != 8) {
		$hash = md5($code . $i);
		$i++;
		if (substr($hash, 0, 5) === '00000') {
			$pos = substr($hash, 5, 1);
			if (!is_numeric($pos) || ($pos > 7)) { continue; }
			
			if (!isset($password[$pos])) {
				$char = substr($hash, 6, 1);
				$password[$pos] = $char;
				echo $i . ': ' . $hash . ' - ' . $pos . ' - ' . $char . PHP_EOL;
			}
		}		
	}
	ksort($password);
	echo implode('', $password);