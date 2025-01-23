extends Logs

# We need to use _process since L2/R2 are analog triggers and send actions multiple times
func _process(_delta: float) -> void:
	if get_parent().visible:
		if Input.is_action_just_pressed("ui_next_subtab"):
			select_next_available()
		if Input.is_action_just_pressed("ui_prev_subtab"):
			select_previous_available()
