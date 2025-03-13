extends AcceptDialog

func _input(event: InputEvent):
	if event.is_action_pressed("ui_close_info_window"):
		queue_free()
