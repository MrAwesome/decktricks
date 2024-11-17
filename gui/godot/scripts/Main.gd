extends Control


# TODO: some kind of error display system
# TODO: fix going up from info sometimes going to tabs instead of previous trick's buttons
# TODO: keep track of selected option between refreshes, or actually replace children individually down to the button level
# TODO: does selecting a node keep it from being cleaned up?
# TODO: fix follow logic on click vs up
# TODO: handle 720p since that's a common resolution on TVs?
# TODO: use this to set the STEAM_ID as needed for gamescope? https://docs.godotengine.org/en/stable/classes/class_window.html#class-window-method-set-flag

var init = true
var ACTIONS_ROW = preload("res://scenes/actions_row.tscn")
var ACTION_BUTTON = preload("res://scenes/action_button.tscn")
var LABEL_OUTER = preload("res://scenes/label_outer.tscn")
var ROW_OUTER = preload("res://scenes/row_outer.tscn")
var TRICKS_LIST = preload("res://scenes/tricks_list.tscn")
var TRICK_INFO = preload("res://scenes/trick_info.tscn")


@onready var display_name_mapping: Dictionary = %DecktricksDispatcher.get_display_name_mapping()
var did_focus = false
var focused_trick_and_action = [null, null]

func focus_button(button: Button, action, trick_id):
	# On button focus, make sure that at least one row above can be focused
	#    (to fix scrolling up inside tabcontainer)
	var row = button.find_parent("MarginContainer").get_parent()
	var idx = row.get_index()
	var desired_idx = max(0, idx-1)
	var desired_row = row.get_parent().get_child(desired_idx)
	%ScrollContainer.ensure_control_visible(desired_row)

	# Store the focused button to be re-focused on refresh
	focused_trick_and_action = [trick_id, action]

func create_action_button(action: String, trick_id: String):
	var button: Button = ACTION_BUTTON.instantiate()
	button.name = action
	button.text = display_name_mapping[action]
	button.pressed.connect(take_action.bind(action, trick_id))
	button.focus_entered.connect(focus_button.bind(button, action, trick_id))
	return button

func take_action(action: String, trick_id: String):
	var args: Array[String] = [action, trick_id]
	if action == "info":
		var output = %DecktricksDispatcher.sync_run_with_decktricks(args)
		if output == "":
			print('Error! Failed to run', './decktricks ', action, ' ', trick_id)
			return

		# TODO: test for extremely long info strings
		# TODO: figure out why keybindings and themes don't work here
		var info_json = JSON.new()
		var ret = info_json.parse(output)
		if ret == OK:
			var info = info_json.data

			var root = get_tree().root
			var dialog = TRICK_INFO.instantiate()
			dialog.theme = theme
			dialog.get_ok_button().set_text("OK")

			dialog.set_title(info["display_name"])
			dialog.set_text(info["description"])

			root.add_child(dialog)
			dialog.popup_centered_ratio(0.7)
	else:
		%DecktricksDispatcher.async_run_with_decktricks(args)

# take [available, actions, like, this]
func create_actions_row(trick_id: String, available_actions, _display_name: String, _icon_path: String):
	var actions_row_outer = ACTIONS_ROW.instantiate()
	var actions_row = actions_row_outer.get_child(0).get_child(0)

	var should_focus = focused_trick_and_action[0] == trick_id

	for action in available_actions:
		# Fix this to take display names etc from the config
		# TODO: fix ordering so info is last
		var button = create_action_button(action, trick_id)
		actions_row.add_child(button)

		if should_focus and action == focused_trick_and_action[1]:
			button.grab_focus.call_deferred()
			did_focus = true

	if should_focus and not did_focus:
		actions_row.get_child(0).grab_focus.call_deferred()
		did_focus = true
	return actions_row_outer

func get_actions():
	# NOTE: we should not need to check validity of this output if it returns successfully,
	# 		thanks to robust error-checking on the Rust side
	var args: Array[String] = ["actions", "--json"]
	var config_data = %DecktricksDispatcher.sync_run_with_decktricks(args)

	var actions_json = JSON.new()
	var ret = actions_json.parse(config_data)

	if ret == OK:
		return actions_json.data
	# TODO: fallback/error

func _ready():
	Engine.set_max_fps(30)
	refresh_ui()

func refresh_ui():
	var first_button = null
	var games = TRICKS_LIST.instantiate()
	var actions = get_actions()

	var args: Array[String] = ["get-config"]
	var config_data = %DecktricksDispatcher.sync_run_with_decktricks(args)

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

			# TODO: show tooltext when it's selected

			var label_box = LABEL_OUTER.instantiate()
			var label = label_box.get_child(0)
			label.text = display_name
			label_box.tooltip_text = description

			var trick_row = create_actions_row(trick_id, available_actions, display_name, icon_path)

			if init and not marked_first:
				first_button = trick_row.get_child(0).get_child(0).get_child(0)
				marked_first = true

			var row_outer_here = ROW_OUTER.instantiate()
			var row_inner = row_outer_here.get_child(0).get_child(0)
			row_inner.add_child(label_box)
			row_inner.add_child(trick_row)
			games.add_child(row_outer_here)

	var old_lists = %ScrollContainer.get_children()

	for l in old_lists:
		%ScrollContainer.remove_child(l)
		l.queue_free()

	%ScrollContainer.add_child(games)

	if init or not did_focus:
		first_button.grab_focus.call_deferred()
	init = false
	did_focus = false

func _input(event: InputEvent) -> void:
	# If this window loses focus, do not accept any input (otherwise,
	# we would process gamepad input while child programs are in focus
	# which is a major problem in gamescope)
	if not DisplayServer.window_is_focused(0):
		accept_event()
		return
	if event.is_action_pressed("ui_exit_decktricks"):
		get_tree().quit()
	# var screen_size: Vector2i = DisplayServer.screen_get_size()
	# print(screen_size.x)

func _on_ui_refresh_timer_timeout() -> void:
	refresh_ui()
