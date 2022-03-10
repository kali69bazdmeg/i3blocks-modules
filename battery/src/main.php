<?php

	$capacity = @file_get_contents("/sys/class/power_supply/BAT0/capacity");
	$status   = @file_get_contents("/sys/class/power_supply/BAT0/status");

	if (intval(trim($capacity)) <= 20) {
		if (trim($status) !== "Discharging") {
			printf("%s %s\n", "", trim($capacity) . "%");
		}
		else {
			if (intval(trim($capacity)) <= 5) {
				printf("%s %s\n", "", trim($capacity) . "%");
			}
			else if (intval(trim($capacity)) <= 10) {
				printf("%s %s\n", "", trim($capacity) . "%");
			}
			else {
				printf("%s %s\n", "", trim($capacity) . "%");
			}
		}
	}
	else if (intval(trim($capacity)) <= 30) {
		if (trim($status) !== "Discharging") {
			printf("%s %s\n", "", trim($capacity) . "%");
		}
		else {
			printf("%s %s\n", "", trim($capacity) . "%");
		}
	}
	else if (intval(trim($capacity)) <= 40) {
		if (trim($status) !== "Discharging") {
			printf("%s %s\n", "", trim($capacity) . "%");
		}
		else {
			printf("%s %s\n", "", trim($capacity) . "%");
		}
	}
	else if (intval(trim($capacity)) <= 60) {
		if (trim($status) !== "Discharging") {
			printf("%s %s\n", "", trim($capacity) . "%");
		}
		else {
			if (intval(trim($capacity)) <= 50) {
				printf("%s %s\n", "", trim($capacity) . "%");
			}
			else {
				printf("%s %s\n", "", trim($capacity) . "%");
			}
		}
	}
	else if (intval(trim($capacity)) <= 80) {
		if (trim($status) !== "Discharging") {
			printf("%s %s\n", "", trim($capacity) . "%");
		}
		else {
			if (intval(trim($capacity)) <= 70) {
				printf("%s %s\n", "", trim($capacity) . "%");
			}
			else {
				printf("%s %s\n", "", trim($capacity) . "%");
			}
		}
	}
	else if (intval(trim($capacity)) <= 90) {
		if (trim($status) !== "Discharging") {
			printf("%s %s\n", "", trim($capacity) . "%");
		}
		else {
			printf("%s %s\n", "", trim($capacity) . "%");
		}
	}
	else if (intval(trim($capacity)) <= 100) {
		if (trim($status) !== "Discharging") {
			printf("%s %s\n", "", trim($capacity) . "%");
		}
		else {
			printf("%s %s\n", "", trim($capacity) . "%");
		}
	}

?>
