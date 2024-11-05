extends Control

var actions_row = preload("res://scenes/actions_row.tscn")
var action_button = preload("res://scenes/action_button.tscn")
var label_outer = preload("res://scenes/label_outer.tscn")
var row_outer = preload("res://scenes/row_outer.tscn")
var tricks_list = preload("res://scenes/tricks_list.tscn")

func create_action_button(action: String, target: String, contents: String):
	var button = action_button.instantiate()
	button.name = action
	button.text = contents
	button.pressed.connect(take_action.bind(action, target))
	return button

func take_action(action: String, target: String):
	# TODO: special handling for "info" etc
	print('Running: ', './decktricks ', action, ' ', target)
	OS.execute_with_pipe("./decktricks", [action, target])
	# TODO: check res

# take [available, actions, like, this]
func create_actions_row(target: String, available_actions, _display_name: String, _icon_path: String):
	var actions_row_outer = actions_row.instantiate()
	var row = actions_row_outer.get_child(0).get_child(0)
	
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
	refresh_ui()
	
func refresh_ui():
	var first_button = null
	var games = tricks_list.instantiate()
	var actions = get_actions()
	
	var config_output = []
	var config_res = OS.execute("./decktricks", ["get-config"], config_output)
	
	if config_res == 0:
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
				
				var label_box = label_outer.instantiate()
				var label = label_box.get_child(0)
				label.text = display_name
				label_box.tooltip_text = description

				var trick_row = create_actions_row(trick_id, available_actions, display_name, icon_path)
				
				if not marked_first:
					first_button = trick_row.get_child(0).get_child(0).get_child(0)
					marked_first = true
				
				var row_outer_here = row_outer.instantiate()
				var row_inner = row_outer_here.get_child(0).get_child(0)
				row_inner.add_child(label_box)
				row_inner.add_child(trick_row)
				games.add_child(row_outer_here)
	
	var old_games = %ScrollContainer.find_child("TricksList")
	if old_games != null:
		%ScrollContainer.remove_child(old_games)
		old_games.queue_free()

	%ScrollContainer.add_child(games)

	first_button.grab_focus()

func _input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_exit_decktricks"):
		get_tree().quit()

func _on_ui_refresh_timer_timeout() -> void:
	refresh_ui()
