<?php

	$time     = explode(":", date("H:m:s:a"));

	$hour     = $time[0];
	$minute   = $time[1];
	$second   = $time[2];
	$meridiem = $time[3];

	if ($meridiem == "am") {
		switch ($hour) {
			case "01":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "02":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "03":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "04":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "05":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "06":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "07":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "08":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "09":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "10":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "11":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "12":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			default:
				printf("%s\n", "Error");
		}
	}
	else if ($meridiem === "pm") {
		switch ($hour) {
			case "13":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "14":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "15":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "16":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "17":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "18":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "19":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "20":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "21":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "22":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "23":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			case "00":
				printf("%s %s:%s:%s\n", "", $hour, $minute, $second);
				break;
			default:
				printf("%s\n", "Error");
		}
	}
	else {
		printf("%s\n", "Error");
	}

?>
