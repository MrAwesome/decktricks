extends Button

func _gui_input(event: InputEvent) -> void:
	if event.is_pressed():
		print(event)
