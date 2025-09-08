extends Button

func _on_pressed() -> void:
	var test_inputs = OS.get_environment("DECKTRICKS_GUI_TEST_INPUTS")
	if test_inputs:
		print("EXITING BY PRESSING EXIT BUTTON")
	get_tree().quit()
