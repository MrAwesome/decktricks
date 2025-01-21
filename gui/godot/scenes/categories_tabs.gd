extends TabContainer

func _input(event: InputEvent) -> void:
	if get_parent().visible:
		if event.is_action_pressed("ui_next_subtab"):
			select_next_available()
		if event.is_action_pressed("ui_prev_subtab"):
			select_previous_available()
