extends Control

# Create function that takes action and display name (or icon)
# Call it for each one

func create_action_button(action: String, target: String, contents: String):
	var button = $Button.duplicate()
	button.name = action
	button.text = contents
	button.pressed.connect(take_action.bind(action, target))
	return button

func take_action(action: String, target: String):
	print('Running: ', './decktricks ', action, ' ', target)
	var output = []
	var res = OS.execute("./decktricks", [action, target], output, true)
	# TODO: check res
	print(output)

# take [available, actions, like, this]
func create_row(target: String, available_actions, _display_name: String, _icon_path: String):
	var row = HBoxContainer.new()
	
	var did_first = false
	for action in available_actions:
		var button = create_action_button(action, target, action) #Fix this to take display names etc from the config
		if not did_first:
			button.name = "first_button"
			did_first = true
		
		row.add_child(button)
	return row

func get_actions():
	var output = []
	var res = OS.execute("./decktricks", ["actions", "--json"], output)
	print(output[0])
	# TODO: check output is legitimate
	
	if res == 0:
		var config_data = output[0]
		var actions_json = JSON.new()
		var ret = actions_json.parse(config_data)
		
		if ret == OK:
			return actions_json.data
		# TODO: fallback/error
	
func _ready():
	var games = $ScrollContainer/MarginContainer/Games
	var actions = get_actions()
	print(actions['chromium'])
	
	var file = FileAccess.open("res://assets/config.json", FileAccess.READ)
	if file:
		var config_data = file.get_as_text()
		file.close()
		var json = JSON.new()
		var ret = json.parse(config_data)
		
		if ret == OK:
			var tricks = json.data.get("tricks", [])
			for trick in tricks:
				var trick_id = trick.get("id")
				var display_name = trick.get("display_name")
				var icon_path = trick.get("icon")
				
				var available_actions = actions[trick_id]
				# TODO: error checking
				
				var TEMP_row_label = $Label.duplicate()
				TEMP_row_label.text = display_name
				games.add_child(TEMP_row_label)

				var trick_row = create_row(trick_id, available_actions, display_name, icon_path)
				games.add_child(trick_row)

	# TODO: improve
	var first_row = games.get_children()[1]
	var first_button = first_row.get_children()[0]
	first_button.grab_focus()

func _input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_exit_decktricks"):
		get_tree().quit()
