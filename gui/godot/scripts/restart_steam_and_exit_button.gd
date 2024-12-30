extends Button

func _on_pressed():
	DecktricksDispatcher.restart_steam()
	get_tree().quit()

func _on_decktricks_restart_steam_hint() -> void:
	visible = true
