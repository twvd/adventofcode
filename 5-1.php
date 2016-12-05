<?php
	$code = $argv[1];
	
	$i = 0;
	$password = '';
	
	while (strlen($password) != 8) {
		$hash = md5($code . $i);
		if (substr($hash, 0, 5) === '00000') {
			$password .= substr($hash, 5, 1);
			echo substr($hash, 5, 1);
		}
		$i++;
	}