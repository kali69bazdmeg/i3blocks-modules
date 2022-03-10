<?php

	$date  = explode(":", date("Y:m:d"));

	$year  = $date[0];
	$month = $date[1];
	$day   = $date[2];

	printf("%s  %s.%s.%s\n", "", $year, $month, $day);

?>
