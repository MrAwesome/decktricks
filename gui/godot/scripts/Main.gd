extends Control

# Create function that takes action and display name (or icon)
# Call it for each one

# TODO: figure out how to remove $Button from the scene so it's not selectable with controller

var first_button = null

func create_action_button(action: String, target: String, contents: String):
	var button = $Hidden/Button.duplicate()
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
func create_actions_row(target: String, available_actions, _display_name: String, _icon_path: String):
	var actions_row_outer = $Hidden/ActionsRowOuter.duplicate()
	var row = actions_row_outer.get_child(0).get_child(0)
	
	var did_first = false
	for action in available_actions:
		# Fix this to take display names etc from the config
		# TODO: fix ordering so info is last
		var button = create_action_button(action, target, action)
		row.add_child(button)
	return actions_row_outer

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
	var games = $MainPanel/ScrollContainer/Games
	var actions = get_actions()
	print(actions['chromium'])
	
	#var file = FileAccess.open("res://assets/config.json", FileAccess.READ)
	var config_output = []
	var config_res = OS.execute("./decktricks", ["get-config"], config_output)
	
	
	if config_res == 0:
		#var config_data = file.get_as_text()
		#file.close()
		var config_data = "".join(config_output)
		var json = JSON.new()
		var ret = json.parse(config_data)
		
		
		
		if ret == OK:
			var tricks = json.data.get("tricks", [])
			
			var marked_first = false
			for trick in tricks:
				var trick_id = trick.get("id")
				var display_name = trick.get("display_name")
				var icon_path = trick.get("icon") # TODO: either make this mandatory or remove it
				var description = trick.get("description")
				
				# Error checking should never be needed for this access, since we
				# check on the Rust side that we're only generating valid actions
				var available_actions = actions[trick_id]
				
				
				# TODO: make label selectable, have it just jump to the first option
				# TODO: show tooltext when it's selected
				
				var label_box = $Hidden/LabelOuter.duplicate()
				var label = label_box.get_child(0)
				label.text = display_name
				label_box.tooltip_text = description

				var trick_row = create_actions_row(trick_id, available_actions, display_name, icon_path)
				
				if not marked_first:
					first_button = trick_row.get_child(0).get_child(0).get_child(0)
					marked_first = true
				
				var row_outer = $Hidden/RowOuter.duplicate()
				var row_inner = row_outer.get_child(0).get_child(0)
				row_inner.add_child(label_box)
				row_inner.add_child(trick_row)
				games.add_child(row_outer)

	print(first_button.text)
	first_button.grab_focus()
	
	$Hidden/Button.get_parent().remove_child($Hidden/Button)

func _input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_exit_decktricks"):
		get_tree().quit()
