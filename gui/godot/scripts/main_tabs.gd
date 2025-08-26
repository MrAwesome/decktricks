extends TabContainer

func _input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_cancel"):
		if get_tab_bar().has_focus():
			%Exit.visible = true
		else:
			get_tab_bar().grab_focus()
