extends Logs

func _input(event: InputEvent) -> void:
	if get_parent().visible:
		if event.is_action_pressed("ui_next_log_tab"):
			%LogContainer.select_next_available()
		if event.is_action_pressed("ui_prev_log_tab"):
			%LogContainer.select_previous_available()
