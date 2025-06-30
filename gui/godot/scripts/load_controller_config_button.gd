extends Button

func _on_pressed() -> void:
	print("Pressed...")
	DecktricksDispatcher.load_controller_config()
