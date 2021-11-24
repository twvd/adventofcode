<?php
// State array: each index represents a floor (0 = floor 1, etc).
// Each floor has 10 positions. The elevator can only be on one floor.
// Each object is a 3-character string; if it ends in a G it is a generator (isGen),
// M is a microchip (isChip).
//
// The first 2 characters are the type (getItemType)

// With all optimizations (never walk down a branch twice, only take 2 items up and 1 down)
// this solves the puzzle in 4.7 seconds (PHP 7.0.8)

define('DOWN', -1);
define('UP', 1);
define('TOPFLOOR', 3);
define('TOTALITEMS', 10);

// Keep a history of state hashes so we don't walk down branches of states that we've already
// progressed on in a different branch of our tree.
$hashHistory = [];

// comb() function from http://stackoverflow.com/a/25611822
// Computes all possible combinations of 'buckets' from array $a of size $m
function comb($m, $a) {
    if (!$m) {
        yield [];
        return;
    }
    if (!$a) {
        return;
    }

    $h = $a[0];
    $t = array_slice($a, 1);
    foreach(comb($m - 1, $t) as $c)
        yield array_merge([$h], $c);
    foreach(comb($m, $t) as $c)
        yield $c;
}

function isChip($s) {
    return (substr($s, 2, 1) == 'M');
}

function isGen($s) {
    return (substr($s, 2, 1) == 'G');
}

function getItemType($s) {
    return (substr($s, 0, 2));
}

// Function that checks if a given state is the end of the problem.
function stateIsEnd($state) {
    // Problem ends when all 10 devices are at floor 4
    return count($state[TOPFLOOR]) == TOTALITEMS;
}

// Function that checks one floor to see if there are conflicting items.
function hasConflictingItems($floor) {
    // Categorize items first
    $generators = [];
    $chips = [];
    foreach ($floor as $item) {
        if (isGen($item)) { $generators []= getItemType($item); }
        if (isChip($item)) { $chips []= getItemType($item); }
    }

    // No chips or no generators is always fine
    if ((count($chips) == 0) || (count($generators) == 0)) { return false; }

    // There are chips, so see if their corresponding generator is here.
    foreach ($chips as $chip) {
        // Chip here without corresponding generator, so this is not allowed.
        if (array_search($chip, $generators) === FALSE) {
            return true;
        }
    }

    return false;
}

// Function to check if a given state is a valid one (e.g. no microchips with the wrong generators)
function isValidMove($state) {
    foreach ($state as $k => $floor) {
        // Skip other state data (current floor, depth)
        if (!is_numeric($k)) { continue; }

        if (hasConflictingItems($floor)) {
            return false;
        }
    }

    return true;
}

// Calculate a hash of a given state using PHP's serialize()
function getStateHash($state) {
    unset($state['floor']);
    unset($state['depth']);
    return serialize($state);
}

// Function to check if a state was previously seen.
function previouslySeenState($state) {
    global $hashHistory;

    $hash = getStateHash($state);

    if (isset($hashHistory[$hash])) {
        return true;
    }

    // Not seen yet, so add it to the hash history now.
    $hashHistory[$hash] = true;
    return false;
}

// Function to calculate the next possible moves based on a state
function calcNextPossibleMoves($state) {
    $results = [];
    $floor = $state['floor'];

    // We can bring one or two things at once, and we can move up and down.
    for ($items = 1; $items <= 2; $items++) {
        if (($items == 2) && (count($state[$floor]) < 2)) {
            // Only one item here, so skip 2 items
            continue;
        }

        foreach ([DOWN, UP] as $direction) {
            // Optimization: only ever take 1 item down.
            if (($direction == DOWN) && ($items == 2)) { continue; }
            // Optimization: only ever take 2 items up.
            if (($direction == UP) && ($items == 1)) { continue; }

            // If we're at the top or bottom floor, we can't move in one of these directions
            if (($direction == DOWN) && ($floor == 0) || ($direction == UP) && ($floor == TOPFLOOR)) {
                continue;
            }

            foreach (comb($items, $state[$floor]) as $group) {
                $newState = $state;

                foreach ($group as $item) {
                    unset($newState[$floor][array_search($item, $newState[$floor])]);
                    $newState[$floor + $direction] []= $item;
                    sort($newState[$floor + $direction]);
                }

                if (isValidMove($newState) && !previouslySeenState($newState)) {
                    $newState['floor'] = $floor + $direction;
                    $newState['depth']++;
                    $results []= $newState;
                }
            }
        }
    }

    return $results;
}

// Helper function to print a given state to the console
function printState($state) {
    for ($floor = 0; $floor <= TOPFLOOR; $floor++) {
        echo 'F' . ($floor + 1) . ' ';
        if ($state['floor'] == $floor) { echo 'E '; }
        else { echo '  '; }

        echo implode(' ', $state[$floor]) . PHP_EOL;
    }
    echo '--- depth: ' . $state['depth'] . PHP_EOL;
}

// The initial state (puzzle input)
$startState = [
    [ 'TLG', 'TLM', 'PLG', 'STG' ],     // Floor 1
    [ 'PLM', 'STM' ],                   // Floor 2
    [ 'PRG', 'PRM', 'RUG', 'RUM' ],     // Floor 3
    [ ],                                // Floor 4 (target)

    'floor' => 0,
    'depth' => 0
];

$queue = [$startState];
$maxDepth = 0;
$movesChecked = 0;
$tStart = microtime(true);

// Run breadth-first search algorithm
// https://en.wikipedia.org/wiki/Breadth-first_search
while (true) {
    $state = array_shift($queue);

    $moves = calcNextPossibleMoves($state);

    foreach ($moves as $move) {
        if ($move['depth'] > $maxDepth) {
            echo 'At depth: ' . $move['depth'] . ', moves so far: ' . $movesChecked . PHP_EOL;
            $maxDepth = $move['depth'];
        }

        if (stateIsEnd($move)) {
            $tEnd = microtime(true);
            echo 'End found!' . PHP_EOL;
            echo 'Moves checked: ' . $movesChecked . ', max depth: ' . $maxDepth . PHP_EOL;
            echo 'Time: ' . ($tEnd - $tStart) . ' seconds' . PHP_EOL;
            printState($move);
            echo PHP_EOL;
            echo 'Puzzle 1 solution: ' . $maxDepth . PHP_EOL;
            echo 'Puzzle 2 solution: ' . ($maxDepth + 24) . PHP_EOL;
            exit();
        }

        $queue []= $move;
        $movesChecked++;
    }
}