extends Control

# TODO: some kind of error display system (emit signal and handle it by displaying an AcceptDialog with the text and a report link/QR, and have a timeout for how many errors can be shown at a time (or how quickly)), and have an exit program option from errors
# TODO: fix going up from info sometimes going to tabs instead of previous trick's buttons
# TODO: only re/populate new nodes on refresh (requires keeping full track of previous state)
# TODO: does selecting a node keep it from being cleaned up?
# TODO: handle 720p since that's a common resolution on TVs?
# TODO: use this to set the STEAM_ID as needed for gamescope? https://docs.godotengine.org/en/stable/classes/class_window.html#class-window-method-set-flag

const DEFAULT_MAX_FPS = 30
var dd = DecktricksDispatcher

var initializing = true
var ACTIONS_ROW = preload("res://scenes/actions_row.tscn")
var ACTION_BUTTON = preload("res://scenes/action_button.tscn")
var LABEL_OUTER = preload("res://scenes/label_outer.tscn")
var ROW_OUTER = preload("res://scenes/row_outer.tscn")
var TRICKS_LIST = preload("res://scenes/tricks_list.tscn")
var INFO_WINDOW = preload("res://scenes/info_window.tscn")

var did_focus = false
var last_actions_string = "THROWAWAY_VALUE"
var focused_trick_and_action = [null, null]

@onready var display_name_mapping: Dictionary = dd.get_display_name_mapping()
@onready var config: Dictionary = get_config()

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

func popup_info_window(root: Window, info: Dictionary):
		var dialog: AcceptDialog = INFO_WINDOW.instantiate()
		dialog.theme = theme
		dialog.get_ok_button().set_text("OK")

		dialog.set_title(info["display_name"])
		dialog.set_text(info["description"])

		root.add_child(dialog)
		dialog.popup_centered_ratio(0.8)

func take_action(action: String, trick_id: String):
	var args: Array[String] = [action, trick_id]
	if action == "info":
		var output = dd.sync_run_with_decktricks(args)
		if output == "":
			print('Error! Failed to run', './decktricks ', action, ' ', trick_id)
			return

		# TODO: test for extremely long info strings
		var info_json = JSON.new()
		var ret = info_json.parse(output)
		if ret == OK:
			var info = info_json.data
			var root = get_tree().root
			popup_info_window(root, info)
	else:
		dd.async_run_with_decktricks(args)

func create_actions_row(trick_id: String, available_actions: Array, _display_name: String, _icon_path: String):
	var actions_row_outer = ACTIONS_ROW.instantiate()
	var actions_row = actions_row_outer.get_child(0).get_child(0)

	var should_focus = focused_trick_and_action[0] == trick_id

	for action in available_actions:
		# Fix this to take display names etc from the config
		var button = create_action_button(action, trick_id)
		actions_row.add_child(button)

		if should_focus and action == focused_trick_and_action[1]:
			button.grab_focus.call_deferred()
			did_focus = true

	if should_focus and not did_focus:
		actions_row.get_child(0).grab_focus.call_deferred()
		did_focus = true
	return actions_row_outer

func get_actions_text_sync():
	# NOTE: we should not need to check validity of this output if it returns successfully,
	# 		thanks to robust error-checking on the Rust side
	var args: Array[String] = ["actions", "--json"]
	var actions_text = dd.sync_run_with_decktricks(args)
	return actions_text

func parse_actions(actions_text: String):
	var json = JSON.new()
	var ret = json.parse(actions_text)

	if ret == OK:
		return json.data
	else:
		print("Error parsing actions JSON: ", actions_text)
		return {}

func get_config():
	# NOTE: we should not need to check validity of this output if it returns successfully,
	# 		thanks to robust error-checking on the Rust side
	#var args: Array[String] = ["get-config"]
	#var config_data = dd.sync_run_with_decktricks(args)
	var config_data = dd.get_config_text()

	var json = JSON.new()
	var ret = json.parse(config_data)

	if ret == OK:
		return json.data
	else:
		print("Error parsing config JSON: ", config_data)
		return {}

# TODO: this should live on TricksList, not here?
func refresh_ui(actions_json_string: String):
	# Deferred so that this can be called from outside the main thread:
	refresh_ui_inner.call_deferred(actions_json_string)

func refresh_ui_inner(actions_json_string: String):
	# Comment this out to test focus behavior on UI change
	if actions_json_string == last_actions_string:
		return

	var actions = parse_actions(actions_json_string)

	var first_button = null
	var games = TRICKS_LIST.instantiate()

	var tricks = config.get("tricks", [])

	var marked_first = false
	for trick in tricks:
		var trick_id = trick.get("id")
		var display_name = trick.get("display_name")
		var icon_path = trick.get("icon") # TODO: either make this mandatory or remove it
		#var description = trick.get("description")

		# Error checking should never be needed for this access, since we
		# check on the Rust side that we're only generating valid actions
		var available_actions: Array = actions[trick_id]

		# TODO: show tooltext when it's selected

		var label_box = LABEL_OUTER.instantiate()
		var label = label_box.get_child(0)
		label.text = display_name

		# These ended up being spammy and buggy and too big
		# label_box.tooltip_text = description

		var trick_row = create_actions_row(trick_id, available_actions, display_name, icon_path)

		if initializing and not marked_first:
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

	if initializing or not did_focus:
		if first_button:
			first_button.grab_focus.call_deferred()
	initializing = false
	did_focus = false

	last_actions_string = actions_json_string

func _init():
	dd.get_time_passed_ms("init")

	# IMPORTANT: this MUST be run before wait_for_executor:
	dd.initialize_executor_with_lock()

	Engine.set_max_fps(DEFAULT_MAX_FPS)

	dd.get_time_passed_ms("init_finished")

func _ready():
	dd.get_time_passed_ms("ready")

	# IMPORTANT: Do not try to run any commands with the executor before this has finished:
	dd.wait_for_executor();
	dd.get_time_passed_ms("executor_ready")

	# Hook up the signal that refreshes our UI over time
	dd.connect("actions", refresh_ui)

	# Synchronously build out our full UI for display
	var actions_text = get_actions_text_sync()
	refresh_ui(actions_text)

	var should_test = OS.get_environment("DECKTRICKS_GUI_TEST_COMMAND_ONLY")
	var should_exit = OS.get_environment("DECKTRICKS_GUI_EXIT_IMMEDIATELY")

	if should_test:
		var test_cmd_args: Array[String]
		test_cmd_args.assign(should_test.split("|DELIM|"))
		dd.sync_run_with_decktricks(test_cmd_args)
	
	%Logs.populate_logs()

	print("Decktricks GUI initialization complete!")
	if should_exit:
		get_tree().quit()

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
# TODO: come up with reasonable values for these timers

func _on_ui_refresh_timer_timeout() -> void:
	dd.async_update_actions()

func _on_log_refresh_timer_timeout() -> void:
	%Logs.populate_logs()

func _on_actions_refresh_timer_timeout() -> void:
	dd.async_executor_refresh()


func _on_exit_tab_focus_entered() -> void:
	print("Would exit")
