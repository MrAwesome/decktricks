extends ColorRect

func _input(event: InputEvent) -> void:
	if visible:
		if event.is_action_pressed("ui_cancel"):
			%ExitButton.grab_focus.call_deferred()
			
