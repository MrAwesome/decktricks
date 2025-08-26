extends Logs

func _input(event: InputEvent) -> void:
	if get_parent().visible:
		if event.is_action_pressed("ui_cancel"):
			if not %MainTabs.get_tab_bar().has_focus():
				if not get_tab_bar().has_focus():
					get_tab_bar().grab_focus()
					get_viewport().set_input_as_handled()

# We need to use _process since L2/R2 are analog triggers and send actions multiple times
func _process(_delta: float) -> void:
	if get_parent().visible:
		if Input.is_action_just_pressed("ui_next_subtab"):
			select_next_available()
			get_tab_bar().grab_focus()

		if Input.is_action_just_pressed("ui_prev_subtab"):
			select_previous_available()
			get_tab_bar().grab_focus()
