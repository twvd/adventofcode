<?php
$instructions = file('10.txt');

$bots = [];
$outputs = [];

// Fill the bots with values first
foreach ($instructions as $line) {
    if (sscanf($line, 'value %d goes to bot %d', $value, $bot)) {
        if (!isset($bots[$bot])) {
            $bots[$bot] = [];
        }
        $bots[$bot] []= $value;
    }
}

$iteration = 0;

// Run the instructions
while (true) {
    $iteration++;

    foreach ($instructions as $line) {
        if (sscanf($line, 'bot %d gives low to %s %d and high to %s %d', 
            $bot, $tlow, $olow, $thigh, $ohigh)) {

            if (!isset($bots[$bot]) || (count($bots[$bot]) < 2)) {
                continue;
            }

            $vlow = min($bots[$bot]);
            $klow = array_search($vlow, $bots);
            $vhigh = max($bots[$bot]);
            $khigh = array_search($vhigh, $bots);

            if ($tlow == 'output') {
                if (!isset($outputs[$olow])) { $outputs[$olow] = []; }
                $outputs[$olow] []= $vlow;
            } else {
                if (!isset($bots[$olow])) { $bots[$olow] = []; }
                $bots[$olow] []= $vlow;
            }

            if ($thigh == 'output') {
                if (!isset($outputs[$ohigh])) { $outputs[$ohigh] = []; }
                $outputs[$ohigh] []= $vhigh;
            } else {
                if (!isset($bots[$ohigh])) { $bots[$ohigh] = []; }
                $bots[$ohigh] []= $vhigh;
            }

            unset($bots[$bot][$klow]);
            unset($bots[$bot][$khigh]);

            if (($vlow == 17) && ($vhigh == 61)) {
                echo 'Finished at iteration ' . $iteration . PHP_EOL;
                echo 'Bot: ' . $bot . PHP_EOL;
                exit();
            }
        }
    }
}