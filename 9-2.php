<?php
	// I designed this with a task/iteration queue to avoid recursion and minimize stack use.
	// Later I also tested the same code but using recursion, which also does
	// not seem to be a problem with the input file I received, but is twice as fast.
	//
	// Time with queue: ~35ms
	// Time with recursion: ~15ms
	
	define('USE_RECURSION', true);
	
	class Task {
		// Offset in the compressed data to start working
		public $offset;
		
		// Length of chunk to parse
		public $length;
		
		// Amount of repetitions needed for this chunk
		public $repeat;
		
		public function __construct($offset, $length, $repeat) {
			$this->offset = $offset;
			$this->length = $length;
			$this->repeat = $repeat;
		}
		
		public function run() {
			global $length, $compressed;
			
			$newtasks = [];
			
			// Using only one string and saving the offsets with the task
			// saves a lot of memory, of course.
			$i = $this->offset;
			$end = $this->offset + $this->length;
			
			while ($i < $end) {
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
	
					// Add the new task to the queue to recurse into this part again
					// We can multiply the repetitions for this task with the new one so we only
					// need to add one new task.
					if (USE_RECURSION) {
						$newtask = new Task($i + $tokenLength, $blockLength, ($repeatCount * $this->repeat));
						$newtask->run();
					} else {
						$newtasks []= new Task($i + $tokenLength, $blockLength, ($repeatCount * $this->repeat));
					}
					
					$i += $tokenLength + $blockLength;
				} else {
					// Just add the single character
					$i++;
					$length += $this->repeat;
				}
			}
			
			return $newtasks;
		}
	};
	
	$tsStart = microtime(true);
	
	$tasks = [];
	$compressed = trim(file_get_contents('9.txt'));
	$length = 0;
	$tasksDone = 0;
	
	// Add first task for the full file
	$tasks []= new Task(0, strlen($compressed), 1);
	
	// Handle every task, while every task can return new tasks to do.
	// These will be added to the end of the task queue.
	while (count($tasks)) {
		$task = array_shift($tasks);
		
		$newtasks = $task->run();
		if (!USE_RECURSION) {
			$tasks = array_merge($tasks, $newtasks);
		}
		$tasksDone++;
	}
	
	$ts = (microtime(true) - $tsStart);
	
	echo 'Answer: ' . $length . PHP_EOL;
	echo 'In ' . $tasksDone . ' iterations, ' . ($ts * 1000) . ' ms' . PHP_EOL;