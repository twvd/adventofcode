<?php
    $compressed = trim(file_get_contents('9.txt'));
    $out = '';
    
    $i = 0;
    while ($i < strlen($compressed)) {
	$c = substr($compressed, $i, 1);
	if ($c == '(') {
	    // We found a compression token
	    preg_match('/\((\d+)x(\d+)\)/', $compressed, $matches, 0, $i);

	    // Total length of the compression token to skip later
	    $tokenLength = strlen($matches[0]);

	    // Length of the block we need to repeat after the token
	    $blockLength = $matches[1];

	    // Amount of times to repeat
	    $repeatCount = $matches[2];

	    $block = substr($compressed, $i + $tokenLength, $blockLength);

	    $out .= str_repeat($block, $repeatCount);

	    // Seek past the token + block we just handled
	    $i += ($blockLength + $tokenLength);
	} else {
	    // Just add the single character
	    $out .= $c;
	    $i++;
	}
    }

    echo strlen($out) . PHP_EOL;