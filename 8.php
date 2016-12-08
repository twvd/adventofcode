<?php
    if (PHP_INT_SIZE != 8) { die('64-bit PHP required'); }

    define('DISPLAY_WIDTH', 50);
    define('DISPLAY_HEIGHT', 6);

    // Initialize (reset) display
    // X-> 0
    //  Y 0..................................................
    //  |  ..................................................
    //  |  ..................................................
    // \_/ ..................................................
    //     ..................................................
    //     ..................................................
    // Each array position is the X. The Y pixels are bitmasks of max 0x3F (DISPLAY_HEIGHT - 1)
    $display = [];
    for ($x = 0; $x < DISPLAY_WIDTH; $x++) {
        $display[$x] = 0;
    }

    // Run the display instructions
    foreach (file('8.txt') as $instruction) {
	if (sscanf($instruction, 'rect %dx%d', $width, $height)) {
	    for ($x = 0; $x < $width; $x++) {
		$display[$x] |= ((1 << $height) - 1);
	    }
	} else if (sscanf($instruction, 'rotate row y=%d by %d', $srcY, $rotBy)) {
	    // Shift across the array, slightly more tricky.
	
	    // Create bitmask to manipulate this row
	    $row = 0;
	    for ($x = 0; $x < DISPLAY_WIDTH; $x++) {
		if ($display[$x] & (1 << $srcY)) {
    		    $row |= (1 << $x);
		}
	    }

	    // Do rotation on the bitmask
	    $rotBy %= DISPLAY_WIDTH;
	    $row = (($row << $rotBy) | ($row >> (DISPLAY_WIDTH - $rotBy)));

	    // Copy bitmask back into row
	    for ($x = 0; $x < DISPLAY_WIDTH; $x++) {
		if ($row & (1 << $x)) { $display[$x] |= (1 << $srcY); }
		else { $display[$x] &= ~(1 << $srcY); }
	    }
	} else if (sscanf($instruction, 'rotate column x=%d by %d', $srcX, $rotBy)) {
	    // Bitwise rotate (ROL) our column

	    $rotBy %= DISPLAY_HEIGHT;
	    $display[$srcX] = (($display[$srcX] << $rotBy) | ($display[$srcX] >> (DISPLAY_HEIGHT - $rotBy)));
	} else {
	    die('Unknown instruction: ' . $instruction);
	}
    }

    function displayPrint() {
	global $display;

	$pixelCount = 0;

        // Print display
	for ($y = 0; $y < DISPLAY_HEIGHT; $y++) {
            for ($x = 0; $x < DISPLAY_WIDTH; $x++) {
    	        echo (($display[$x] & (1 << $y)) ? '#' : '.');
		if ($display[$x] & (1 << $y)) {
		    $pixelCount++;
		}
    	    }
    	    echo PHP_EOL;
	}

	echo $pixelCount . ' pixels' . PHP_EOL;
    }

    displayPrint();
